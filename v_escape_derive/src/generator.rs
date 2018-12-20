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
        if self.simd {
            self.write_sse(buf);
            self.write_avx(buf);
        }
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
                    // 2 escapes
                    ranges.push(self.pairs.first().unwrap().char);
                    ranges.push(self.pairs.last().unwrap().char);
                    ranges.push(FLAG);
                } else {
                    let i = d.first().unwrap().0;
                    if i != 0 {
                        // 1 escape and 1 range
                        ranges.push(self.pairs.first().unwrap().char);
                        ranges.push(self.pairs.get(i).unwrap().char);
                        ranges.push(self.pairs.get(i + 1).unwrap().char);
                        if i + 2 != len {
                            // 2 ranges
                            ranges.push(self.pairs.last().unwrap().char);
                        }
                    } else {
                        // 1 escape and 1 range
                        ranges.push(self.pairs.get(i + 1).unwrap().char);
                        ranges.push(self.pairs.last().unwrap().char);
                        ranges.push(self.pairs.first().unwrap().char);
                    }
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

                if last == first + 1 {
                    if first == 0 {
                        // 1 ranges and 2 escape
                        ranges.push(self.pairs.get(last + 1).unwrap().char);
                        ranges.push(self.pairs.last().unwrap().char);
                        ranges.push(self.pairs.first().unwrap().char);
                        ranges.push(self.pairs.get(first + 1).unwrap().char);
                        ranges.push(FLAG);
                    } else {
                        if last + 2 == len {
                            // 1 ranges and 2 escape
                            ranges.push(self.pairs.first().unwrap().char);
                            ranges.push(self.pairs.get(first).unwrap().char);
                            ranges.push(self.pairs.get(last).unwrap().char);
                            ranges.push(self.pairs.last().unwrap().char);
                            ranges.push(FLAG);
                        } else {
                            // 2 ranges and 1 escape
                            ranges.push(self.pairs.first().unwrap().char);
                            ranges.push(self.pairs.get(first).unwrap().char);
                            ranges.push(self.pairs.get(last + 1).unwrap().char);
                            ranges.push(self.pairs.last().unwrap().char);
                            ranges.push(self.pairs.get(first + 1).unwrap().char);
                        }
                    }
                } else {
                    if first == 0 {
                        if last + 2 == len {
                            // 1 ranges and 2 escape
                            ranges.push(self.pairs.get(first + 1).unwrap().char);
                            ranges.push(self.pairs.get(last).unwrap().char);
                            ranges.push(self.pairs.first().unwrap().char);
                            ranges.push(self.pairs.last().unwrap().char);
                            ranges.push(FLAG);
                        } else {
                            // 2 ranges and 1 escape
                            ranges.push(self.pairs.get(first + 1).unwrap().char);
                            ranges.push(self.pairs.get(last).unwrap().char);
                            ranges.push(self.pairs.get(last + 1).unwrap().char);
                            ranges.push(self.pairs.last().unwrap().char);
                            ranges.push(self.pairs.first().unwrap().char);
                        }
                    } else if len == last + 2 {
                        // 2 ranges and 1 escape
                        ranges.push(self.pairs.first().unwrap().char);
                        ranges.push(self.pairs.get(first).unwrap().char);
                        ranges.push(self.pairs.get(first + 1).unwrap().char);
                        ranges.push(self.pairs.get(last).unwrap().char);
                        ranges.push(self.pairs.last().unwrap().char);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::Pair;

    static E: &[u8] = b"f";

    #[test]
    fn test_1_escape() {
        let pairs = &[Pair::new(0, E)];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 128])
    }

    #[test]
    fn test_2_escape() {
        let pairs = &[Pair::new(0, E), Pair::new(2, E)];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 2, 128])
    }

    #[test]
    fn test_3_escape() {
        let pairs = &[Pair::new(0, E), Pair::new(2, E), Pair::new(4, E)];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 2, 4, 128])
    }

    #[test]
    fn test_1_range() {
        let pairs = &[Pair::new(0, E), Pair::new(1, E)];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 1])
    }

    #[test]
    fn test_2_range() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(3, E),
            Pair::new(4, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 1, 3, 4])
    }

    #[test]
    fn test_3_range() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(3, E),
            Pair::new(4, E),
            Pair::new(6, E),
            Pair::new(7, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 1, 3, 4, 6, 7]);
        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(3, E),
            Pair::new(5, E),
            Pair::new(7, E),
            Pair::new(9, E),
            Pair::new(50, E),
            Pair::new(52, E),
            Pair::new(55, E),
            Pair::new(60, E),
            Pair::new(61, E),
            Pair::new(62, E),
            Pair::new(63, E),
            Pair::new(64, E),
            Pair::new(126, E),
            Pair::new(127, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 9, 50, 64, 126, 127])
    }

    #[test]
    fn test_1_range_1_escape() {
        let pairs = &[Pair::new(0, E), Pair::new(1, E), Pair::new(3, E)];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 1, 3]);

        let pairs = &[Pair::new(0, E), Pair::new(2, E), Pair::new(3, E)];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![2, 3, 0]);

        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(2, E),
            Pair::new(4, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 2, 4]);

        let pairs = &[
            Pair::new(50, E),
            Pair::new(51, E),
            Pair::new(52, E),
            Pair::new(53, E),
            Pair::new(54, E),
            Pair::new(55, E),
            Pair::new(67, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![50, 55, 67]);
    }

    #[test]
    fn test_2_range_1_escape_a() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(3, E),
            Pair::new(4, E),
            Pair::new(6, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 1, 3, 4, 6]);
    }

    #[test]
    fn test_2_range_1_escape_b() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(4, E),
            Pair::new(5, E),
            Pair::new(7, E),
            Pair::new(8, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![4, 5, 7, 8, 0]);
    }

    #[test]
    fn test_2_range_1_escape_c() {
        let pairs = &[
            Pair::new(14, E),
            Pair::new(15, E),
            Pair::new(16, E),
            Pair::new(50, E),
            Pair::new(51, E),
            Pair::new(52, E),
            Pair::new(98, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![14, 16, 50, 52, 98]);
    }

    #[test]
    fn test_2_range_1_escape_d() {
        let pairs = &[
            Pair::new(14, E),
            Pair::new(15, E),
            Pair::new(16, E),
            Pair::new(50, E),
            Pair::new(51, E),
            Pair::new(52, E),
            Pair::new(98, E),
        ];

        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![14, 16, 50, 52, 98]);
        let pairs = &[
            Pair::new(14, E),
            Pair::new(15, E),
            Pair::new(16, E),
            Pair::new(17, E),
            Pair::new(18, E),
            Pair::new(19, E),
            Pair::new(50, E),
            Pair::new(51, E),
            Pair::new(52, E),
            Pair::new(53, E),
            Pair::new(54, E),
            Pair::new(55, E),
            Pair::new(56, E),
            Pair::new(57, E),
            Pair::new(58, E),
            Pair::new(98, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![14, 19, 50, 58, 98]);
    }

    #[test]
    fn test_2_range_1_escape_e() {
        let pairs = &[
            Pair::new(14, E),
            Pair::new(16, E),
            Pair::new(50, E),
            Pair::new(51, E),
            Pair::new(52, E),
            Pair::new(98, E),
        ];

        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![14, 16, 50, 52, 98]);
        let pairs = &[
            Pair::new(14, E),
            Pair::new(16, E),
            Pair::new(17, E),
            Pair::new(18, E),
            Pair::new(19, E),
            Pair::new(50, E),
            Pair::new(52, E),
            Pair::new(53, E),
            Pair::new(56, E),
            Pair::new(57, E),
            Pair::new(58, E),
            Pair::new(98, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![14, 19, 50, 58, 98]);
    }

    #[test]
    fn test_2_range_1_escape_f() {
        let pairs = &[
            Pair::new(60, E),
            Pair::new(61, E),
            Pair::new(65, E),
            Pair::new(80, E),
            Pair::new(81, E),
        ];

        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![60, 61, 80, 81, 65]);

        let pairs = &[
            Pair::new(52, E),
            Pair::new(53, E),
            Pair::new(56, E),
            Pair::new(58, E),
            Pair::new(60, E),
            Pair::new(61, E),
            Pair::new(62, E),
            Pair::new(80, E),
            Pair::new(101, E),
            Pair::new(102, E),
            Pair::new(103, E),
            Pair::new(104, E),
            Pair::new(105, E),
            Pair::new(108, E),
            Pair::new(110, E),
            Pair::new(120, E),
        ];

        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![52, 62, 101, 120, 80]);
    }

    #[test]
    fn test_1_range_2_escape_a() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(4, E),
            Pair::new(6, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 1, 4, 6, 128]);

        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(2, E),
            Pair::new(3, E),
            Pair::new(4, E),
            Pair::new(5, E),
            Pair::new(6, E),
            Pair::new(7, E),
            Pair::new(8, E),
            Pair::new(9, E),
            Pair::new(10, E),
            Pair::new(11, E),
            Pair::new(12, E),
            Pair::new(13, E),
            Pair::new(14, E),
            Pair::new(73, E),
            Pair::new(127, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 14, 73, 127, 128]);
    }

    #[test]
    fn test_1_range_2_escape_b() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(2, E),
            Pair::new(5, E),
            Pair::new(6, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![5, 6, 0, 2, 128]);

        let pairs = &[
            Pair::new(0, E),
            Pair::new(2, E),
            Pair::new(5, E),
            Pair::new(6, E),
            Pair::new(7, E),
            Pair::new(8, E),
            Pair::new(9, E),
            Pair::new(10, E),
            Pair::new(11, E),
            Pair::new(12, E),
            Pair::new(13, E),
            Pair::new(14, E),
            Pair::new(15, E),
            Pair::new(16, E),
            Pair::new(17, E),
            Pair::new(18, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![5, 18, 0, 2, 128]);
    }

    #[test]
    fn test_1_range_2_escape_c() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(2, E),
            Pair::new(3, E),
            Pair::new(8, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![2, 3, 0, 8, 128]);

        let pairs = &[
            Pair::new(0, E),
            Pair::new(2, E),
            Pair::new(3, E),
            Pair::new(4, E),
            Pair::new(5, E),
            Pair::new(6, E),
            Pair::new(7, E),
            Pair::new(8, E),
            Pair::new(9, E),
            Pair::new(10, E),
            Pair::new(11, E),
            Pair::new(12, E),
            Pair::new(13, E),
            Pair::new(14, E),
            Pair::new(15, E),
            Pair::new(16, E),
            Pair::new(17, E),
            Pair::new(127, E),
        ];
        let g = Generator::new(pairs, false, false, false);

        assert_eq!(g.calculate_ranges(), vec![2, 17, 0, 127, 128]);
    }
}
