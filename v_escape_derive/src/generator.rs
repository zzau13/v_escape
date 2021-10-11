use std::fmt::{Display, Write};
use std::str;

use quote::quote;

use crate::Pair;

type Ranges = Vec<u8>;

struct Generator<'a> {
    pairs: &'a [Pair],
    simd: bool,
    avx: bool,
}

pub(crate) fn generate(pairs: &[Pair], simd: bool, avx: bool) -> String {
    Generator::new(pairs, simd, avx).build()
}

impl<'a> Generator<'a> {
    pub fn new(pairs: &[Pair], simd: bool, avx: bool) -> Generator {
        Generator { pairs, simd, avx }
    }

    pub fn build(&self) -> String {
        let mut buf = String::new();

        self.write_static_table(&mut buf);
        self.write_functions(&mut buf);
        self.write_cfg_if(&mut buf);

        buf
    }

    fn write_static_table(&self, buf: &mut String) {
        let len = self.pairs.len();
        let quote = &self.pairs[0].quote;

        if len == 1 {
            buf.push_str(&format!("const V_ESCAPE_CHAR: u8 = {};", self.pairs[0].ch));
            buf.push_str(&format!("static V_ESCAPE_QUOTES: &str = {:#?};", quote));
        } else {
            buf.push_str("static V_ESCAPE_TABLE: [u8; 256] = [");
            for i in 0..=255_u8 {
                let n = self.pairs.binary_search_by(|s| s.ch.cmp(&i)).unwrap_or(len);
                buf.push_str(&format!("{}, ", n))
            }
            buf.push_str("];");

            let quotes: Vec<&str> = self.pairs.iter().map(|s| s.quote.as_str()).collect();
            buf.push_str(&format!(
                "static V_ESCAPE_QUOTES: [&str; {}] = {:#?};",
                len, quotes
            ));
        }

        buf.push_str(&format!("const V_ESCAPE_LEN: usize = {};", len));
    }

    fn write_functions(&self, buf: &mut String) {
        self.write_scalar(buf);
        self.write_ch(buf);
        if self.simd {
            self.write_ranges(buf);
        }
    }

    fn write_ch(&self, buf: &mut String) {
        let code = if self.pairs.len() == 1 {
            quote!(
                mod chars {
                    use super::*;
                    v_escape::escape_char!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                    v_escape::escape_char_ptr!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                    v_escape::escape_char_bytes!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                }
            )
        } else {
            quote!(
                mod chars {
                    use super::*;
                    v_escape::escape_char!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                    v_escape::escape_char_ptr!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                    v_escape::escape_char_bytes!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                }
            )
        };
        buf.push_str(&code.to_string());
    }

    fn write_scalar(&self, buf: &mut String) {
        let code = if self.pairs.len() == 1 {
            quote!(
                mod scalar {
                    use super::*;
                    v_escape::escape_scalar!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                    v_escape::escape_scalar_ptr!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                    v_escape::escape_scalar_bytes!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                }
            )
        } else {
            quote!(
                mod scalar {
                    use super::*;
                    v_escape::escape_scalar!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                    v_escape::escape_scalar_ptr!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                    v_escape::escape_scalar_bytes!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                }
            )
        };
        buf.push_str(&code.to_string());
    }

    fn write_ranges(&self, buf: &mut String) {
        buf.push_str(r#"#[cfg(all(target_arch = "x86_64", not(v_escape_nosimd)))]"#);
        buf.push_str("mod ranges {");

        let ranges: &[u8] = &self.calculate_ranges();

        let t: &[&str] = if self.avx { &["avx", "sse"] } else { &["sse"] };

        for i in t {
            buf.push_str("pub mod ");
            buf.push_str(i);
            buf.push_str(" {");
            buf.push_str("use super::super::*;");
            buf.push_str("v_escape::escape_ranges!(");
            buf.push_str(i);
            if self.pairs.len() == 1 {
                buf.push_str("2 (V_ESCAPE_CHAR, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            } else {
                buf.push_str("2 (V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            }
            self.write_macro_tt(buf, ranges);
            buf.push_str(");");
            buf.push_str("v_escape::escape_ranges_ptr!(");
            buf.push_str(i);
            if self.pairs.len() == 1 {
                buf.push_str("2 (V_ESCAPE_CHAR, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            } else {
                buf.push_str("2 (V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            }
            self.write_macro_tt(buf, ranges);
            buf.push_str(");");
            buf.push_str("v_escape::escape_ranges_bytes!(");
            buf.push_str(i);
            if self.pairs.len() == 1 {
                buf.push_str("2 (V_ESCAPE_CHAR, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            } else {
                buf.push_str("2 (V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            }
            self.write_macro_tt(buf, ranges);
            buf.push_str(");");
            buf.push('}');
        }
        buf.push('}');
    }

    fn write_cfg_if(&self, buf: &mut String) {
        buf.push_str(&format!(
            "v_escape::cfg_escape!({}, {});",
            self.simd, self.avx
        ));
        buf.push_str(&format!(
            "v_escape::cfg_escape_ptr!({}, {});",
            self.simd, self.avx
        ));
        buf.push_str(&format!(
            "v_escape::cfg_escape_bytes!({}, {});",
            self.simd, self.avx
        ));
    }

    fn write_macro_tt<T, I>(&self, buf: &mut String, i: I)
    where
        T: Display,
        I: IntoIterator<Item = T>,
    {
        for c in i.into_iter() {
            buf.write_fmt(format_args!("{}, ", c)).unwrap();
        }
    }

    fn calculate_ranges(&self) -> Ranges {
        assert_ne!(self.pairs.len(), 0);
        let mut ranges: Ranges = vec![];

        if self.pairs.len() == 1 {
            ranges.push(self.pairs[0].ch);
            ranges.push(FLAG);

            return ranges;
        }
        let e = self.pairs.len() - 1;

        let mut d = vec![];
        for i in 0..e {
            let diff = self.pairs[i + 1].ch - self.pairs[i].ch;
            if 1 < diff {
                d.push((i, diff));
            }
        }
        d.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        match d.len() {
            0 => {
                // 1 range
                ranges.push(self.pairs[0].ch);
                ranges.push(self.pairs[e].ch);
            }
            1 => {
                if e == 1 {
                    // 2 equals
                    ranges.push(self.pairs[0].ch);
                    ranges.push(self.pairs[e].ch);
                    ranges.push(FLAG);
                } else {
                    let i = d[0].0;
                    if i == 0 {
                        // 1 equal and 1 range
                        ranges.push(self.pairs[i + 1].ch);
                        ranges.push(self.pairs[e].ch);
                        ranges.push(self.pairs[0].ch);
                    } else {
                        // 1 equal and 1 range
                        ranges.push(self.pairs[0].ch);
                        ranges.push(self.pairs[i].ch);
                        ranges.push(self.pairs[i + 1].ch);
                        if i + 1 != e {
                            // 2 ranges
                            ranges.push(self.pairs[e].ch);
                        }
                    }
                }
            }
            _ => {
                if e <= 2 {
                    assert_eq!(e, 2);
                    // 3 escapes
                    for Pair { ch, .. } in self.pairs {
                        ranges.push(*ch);
                    }
                    ranges.push(FLAG);

                    return ranges;
                }

                let (d, _) = d.split_at_mut(2);
                d.sort_unstable_by_key(|d| d.0);

                let f = d[0].0;
                let l = d[1].0;

                if f + 1 == l {
                    if f == 0 {
                        self.push_1_ranges_2_equals_at_first(&mut ranges, f, l, e);
                    } else if l + 1 == e {
                        self.push_1_ranges_2_equals_at_last(&mut ranges, f, l, e);
                    } else {
                        self.push_2_ranges_1_equals(&mut ranges, f, l, e);
                    }
                } else if f == 0 {
                    if l + 1 == e {
                        self.push_1_ranges_2_equals_at_first_last(&mut ranges, f, l, e);
                    } else {
                        self.push_2_ranges_1_equals_at_first(&mut ranges, f, l, e);
                    }
                } else if l + 1 == e {
                    self.push_2_ranges_1_equals_at_last(&mut ranges, f, l, e);
                } else {
                    self.push_3_ranges(&mut ranges, f, l, e);
                }
            }
        };

        ranges
    }

    fn push_1_ranges_2_equals_at_first(&self, r: &mut Ranges, f: usize, l: usize, e: usize) {
        r.push(self.pairs[l + 1].ch);
        r.push(self.pairs[e].ch);
        r.push(self.pairs[0].ch);
        r.push(self.pairs[f + 1].ch);
        r.push(FLAG);
    }

    fn push_1_ranges_2_equals_at_last(&self, r: &mut Ranges, f: usize, l: usize, e: usize) {
        r.push(self.pairs[0].ch);
        r.push(self.pairs[f].ch);
        r.push(self.pairs[l].ch);
        r.push(self.pairs[e].ch);
        r.push(FLAG);
    }

    fn push_1_ranges_2_equals_at_first_last(&self, r: &mut Ranges, f: usize, l: usize, e: usize) {
        r.push(self.pairs[f + 1].ch);
        r.push(self.pairs[l].ch);
        r.push(self.pairs[0].ch);
        r.push(self.pairs[e].ch);
        r.push(FLAG);
    }

    fn push_2_ranges_1_equals(&self, r: &mut Ranges, f: usize, l: usize, e: usize) {
        r.push(self.pairs[0].ch);
        r.push(self.pairs[f].ch);
        r.push(self.pairs[l + 1].ch);
        r.push(self.pairs[e].ch);
        r.push(self.pairs[f + 1].ch);
    }

    fn push_2_ranges_1_equals_at_first(&self, r: &mut Ranges, f: usize, l: usize, e: usize) {
        r.push(self.pairs[f + 1].ch);
        r.push(self.pairs[l].ch);
        r.push(self.pairs[l + 1].ch);
        r.push(self.pairs[e].ch);
        r.push(self.pairs[0].ch);
    }

    fn push_2_ranges_1_equals_at_last(&self, r: &mut Ranges, f: usize, l: usize, e: usize) {
        r.push(self.pairs[0].ch);
        r.push(self.pairs[f].ch);
        r.push(self.pairs[f + 1].ch);
        r.push(self.pairs[l].ch);
        r.push(self.pairs[e].ch);
    }

    fn push_3_ranges(&self, r: &mut Ranges, f: usize, l: usize, e: usize) {
        r.push(self.pairs[0].ch);
        r.push(self.pairs[f].ch);
        r.push(self.pairs[f + 1].ch);
        r.push(self.pairs[l].ch);
        r.push(self.pairs[l + 1].ch);
        r.push(self.pairs[e].ch);
    }
}

// End flag for indicate more escapes than ranges
const FLAG: u8 = 128;

#[cfg(test)]
mod test {
    use super::*;
    use crate::Pair;

    static E: &str = "f";

    #[test]
    fn test_1_escape() {
        let pairs = &[Pair::new(0, E)];
        let g = Generator::new(pairs, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 128])
    }

    #[test]
    fn test_2_escape() {
        let pairs = &[Pair::new(0, E), Pair::new(2, E)];
        let g = Generator::new(pairs, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 2, 128])
    }

    #[test]
    fn test_3_escape() {
        let pairs = &[Pair::new(0, E), Pair::new(2, E), Pair::new(4, E)];
        let g = Generator::new(pairs, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 2, 4, 128])
    }

    #[test]
    fn test_1_range() {
        let pairs = &[Pair::new(0, E), Pair::new(1, E)];
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 9, 50, 64, 126, 127])
    }

    #[test]
    fn test_1_range_1_escape() {
        let pairs = &[Pair::new(0, E), Pair::new(1, E), Pair::new(3, E)];
        let g = Generator::new(pairs, false, false);

        assert_eq!(g.calculate_ranges(), vec![0, 1, 3]);

        let pairs = &[Pair::new(0, E), Pair::new(2, E), Pair::new(3, E)];
        let g = Generator::new(pairs, false, false);

        assert_eq!(g.calculate_ranges(), vec![2, 3, 0]);

        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(2, E),
            Pair::new(4, E),
        ];
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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

        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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

        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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

        let g = Generator::new(pairs, false, false);

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

        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

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
        let g = Generator::new(pairs, false, false);

        assert_eq!(g.calculate_ranges(), vec![2, 17, 0, 127, 128]);
    }
}
