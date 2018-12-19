use std::str;

use crate::parser::Pair;

struct Generator<'a> {
    pairs: &'a [Pair<'a>],
    sized: bool,
    simd: bool,
    avx: bool,
}

pub fn generate(pairs: &[Pair], sized: bool, simd: bool, avx: bool) -> String {
    Generator::new(pairs, sized, simd, avx).build()
}

impl<'a> Generator<'a> {
    pub fn new<'n>(pairs: &'n [Pair<'n>], sized: bool, simd: bool, avx: bool) -> Generator<'n> {
        Generator {
            pairs,
            sized,
            simd,
            avx,
        }
    }

    pub fn build(&self) -> String {
        let mut buf = Buffer::new(0);

        self.write_static_table(&mut buf);
        self.write_functions(&mut buf);
        self.write_cfg_if(&mut buf);

        buf.buf
    }

    fn write_static_table(&self, buf: &mut Buffer) {
        let len = self.pairs.len();

        buf.write("static V_ESCAPE_TABLE: [u8; 256] = [");
        for i in 0..=255 as u8 {
            let n = self
                .pairs
                .binary_search_by(|s| s.char.cmp(&i))
                .unwrap_or(len);
            buf.write(&format!("{}, ", n))
        }
        buf.writeln("];");

        let quotes: Vec<&str> = self
            .pairs
            .iter()
            .map(|s| str::from_utf8(s.quote).unwrap())
            .collect();
        buf.writeln(&format!(
            "static V_ESCAPE_QUOTES: [&str; {}] = {:#?};",
            len, quotes
        ));

        buf.writeln(&format!("const V_ESCAPE_QUOTES_LEN: usize = {};", len));

        if self.sized {
            buf.write("static V_ESCAPE_SIZED: [u8; 256] = [");
            for i in 0..=255 as u8 {
                let n = self
                    .pairs
                    .binary_search_by(|s| s.char.cmp(&i))
                    .map(|s| self.pairs.get(s).unwrap().quote.len() - 1)
                    .unwrap_or(0);
                buf.write(&format!("{}, ", n))
            }
            buf.writeln("];");
        }
    }

    fn write_functions(&self, buf: &mut Buffer) {
        self.write_scalar(buf);
        self.write_sse(buf);
        self.write_avx(buf);
    }

    fn write_scalar(&self, buf: &mut Buffer) {
        buf.writeln("mod scalar {");
        buf.writeln("use super::*;");
        buf.writeln(
            " _v_escape_escape_scalar!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_QUOTES_LEN);",
        );
        if self.sized {
            buf.writeln(" _v_escape_sized_scalar!(V_ESCAPE_SIZED);");
        }
        buf.writeln("}");
    }

    fn write_sse(&self, buf: &mut Buffer) {
        buf.writeln(
            r#"#[cfg(all(target_arch = "x86_64", not(all(target_os = "windows", v_escape_nosimd))))]"#,
        );
        buf.writeln("mod sse {");
        buf.writeln("use super::*;");

        let mut chars: Vec<u8> = self.pairs.iter().map(|s| s.char).collect();
        assert!(chars.len() <= 16);
        let r = 16 - chars.len();
        chars.extend(vec![0; r]);
        let chars: &[u8] = &chars;

        buf.write(" _v_escape_escape_sse!((V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_QUOTES_LEN) ");
        for c in chars {
            buf.write(&format!("{}, ", c))
        }
        buf.writeln(");");

        if self.sized {
            buf.write(" _v_escape_sized_sse!((V_ESCAPE_SIZED, V_ESCAPE_QUOTES_LEN) ");
            for c in chars {
                buf.write(&format!("{}, ", c))
            }
            buf.writeln(");");
        }

        buf.writeln("}");
    }

    fn write_avx(&self, buf: &mut Buffer) {
        buf.writeln(
            r#"#[cfg(all(target_arch = "x86_64", not(all(target_os = "windows", v_escape_nosimd))))]"#,
        );
        buf.writeln("mod avx {");
        buf.writeln("use super::*;");

        let ranges: &[u8] = &self.calculate_ranges();

        buf.write("_v_escape_escape_avx!((V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_QUOTES_LEN) ");
        for c in ranges {
            buf.write(&format!("{}, ", c))
        }
        buf.writeln(");");

        if self.sized {
            buf.write("_v_escape_sized_avx!((V_ESCAPE_SIZED) ");
            for c in ranges {
                buf.write(&format!("{}, ", c))
            }
            buf.writeln(");");
        }
        buf.writeln("}");
    }

    fn write_cfg_if(&self, buf: &mut Buffer) {
        buf.writeln(&format!(
            "_v_escape_cfg_escape!({}, {});",
            self.simd, self.avx
        ));
        if self.sized {
            buf.writeln(&format!(
                "_v_escape_cfg_sized!({}, {});",
                self.simd, self.avx
            ));
        }
    }

    fn calculate_ranges(&self) -> Vec<u8> {
        // End flag for indicate more escapes than ranges
        const FLAG: u8 = 128;

        let len = self.pairs.len();
        let mut ranges: Vec<u8> = vec![];

        assert_ne!(len, 0);

        if len == 1 {
            ranges.push(self.pairs.get(0).unwrap().char);
            ranges.push(FLAG);

            return ranges;
        }

        let mut d = vec![];
        for i in 0..len - 1 {
            let diff = self.pairs.get(i + 1).unwrap().char - self.pairs.get(i).unwrap().char;
            if 1 < diff {
                d.push((i, diff));
            }
        }
        d.sort_by(|a, b| b.1.cmp(&a.1));

        match d.len() {
            0 => {
                // 1 range
                ranges.push(self.pairs.first().unwrap().char);
                ranges.push(self.pairs.last().unwrap().char);
            }
            1 => {
                if len == 2 {
                    // 2 escapes at distance > 1
                    ranges.push(self.pairs.first().unwrap().char);
                    ranges.push(self.pairs.last().unwrap().char);
                    ranges.push(FLAG);
                } else {
                    // 2 ranges or 1 escape and 1 range at distance > 1
                    ranges.push(self.pairs.first().unwrap().char);
                    let i = d.first().unwrap().0;
                    ranges.push(self.pairs.get(i).unwrap().char);
                    if let Some(pair) = self.pairs.get(i + 1) {
                        ranges.push(pair.char);
                        if let Some(pair) = self.pairs.last() {
                            ranges.push(pair.char);
                        }
                    };
                }
            }
            _ => {
                if self.pairs.len() <= 3 {
                    assert_eq!(self.pairs.len(), 3);
                    // 3 escapes
                    let c: Vec<u8> = self.pairs.iter().map(|s| s.char).collect();
                    ranges.extend(c);
                    ranges.push(FLAG);

                    return ranges;
                }

                let (d, _) = d.split_at_mut(2);
                d.sort_by(|a, b| a.0.cmp(&b.0));

                let first = d.first().unwrap().0;
                let last = d.last().unwrap().0;

                if last - 1 == first {
                    if first == 0 {
                        // 2 escapes and 1 ranges
                        ranges.push(self.pairs.get(last + 1).unwrap().char);
                        ranges.push(self.pairs.last().unwrap().char);
                        ranges.push(self.pairs.first().unwrap().char);
                        ranges.push(self.pairs.get(last).unwrap().char);
                        ranges.push(FLAG);
                    } else {
                        // 2 ranges and 1 escape
                        ranges.push(self.pairs.first().unwrap().char);
                        ranges.push(self.pairs.get(first).unwrap().char);
                        ranges.push(self.pairs.get(last + 1).unwrap().char);
                        ranges.push(self.pairs.last().unwrap().char);
                        ranges.push(self.pairs.get(last).unwrap().char);
                    }
                } else {
                    if first == 0 {
                        // 2 ranges and 1 escape at init
                        ranges.push(self.pairs.get(first + 1).unwrap().char);
                        ranges.push(self.pairs.get(last).unwrap().char);
                        ranges.push(self.pairs.get(last + 1).unwrap().char);
                        ranges.push(self.pairs.last().unwrap().char);
                        ranges.push(self.pairs.first().unwrap().char);
                    } else {
                        // 3 ranges
                        ranges.push(self.pairs.first().unwrap().char);
                        ranges.push(self.pairs.get(first).unwrap().char);
                        ranges.push(self.pairs.get(first + 1).unwrap().char);
                        ranges.push(self.pairs.get(last).unwrap().char);
                        ranges.push(self.pairs.get(last + 1).unwrap().char);
                        ranges.push(self.pairs.last().unwrap().char);
                    }
                }
            }
        };

        ranges
    }
}

struct Buffer {
    // The buffer to generate the code into
    buf: String,
    // The current level of indentation (in spaces)
    indent: u8,
    // Whether the output buffer is currently at the start of a line
    start: bool,
}

impl Buffer {
    fn new(indent: u8) -> Self {
        Self {
            buf: String::new(),
            indent,
            start: true,
        }
    }

    fn writeln(&mut self, s: &str) {
        if s == "}" {
            self.dedent();
        }
        if !s.is_empty() {
            self.write(s);
        }
        self.buf.push('\n');
        if s.ends_with('{') {
            self.indent();
        }
        self.start = true;
    }

    fn write(&mut self, s: &str) {
        if self.start {
            for _ in 0..(self.indent * 4) {
                self.buf.push(' ');
            }
            self.start = false;
        }
        self.buf.push_str(s);
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn dedent(&mut self) {
        if self.indent == 0 {
            panic!("dedent() called while indentation == 0");
        }
        self.indent -= 1;
    }
}
