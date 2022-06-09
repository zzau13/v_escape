use std::collections::BTreeMap;
use std::fmt::{Display, Write};
use std::fs;
use std::path::Path;
use std::str;

use crate::ranges::Switch;
use proc_macro2::{Ident, TokenStream};
use serde::Serialize;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bang, Paren};
use syn::{parenthesized, Token};
use toml::Value;

#[derive(Serialize)]
struct Dep {
    version: &'static str,
    optional: bool,
}

pub fn generate<P: AsRef<Path>>(dir: P) {
    let dir = dir.as_ref();
    assert!(dir.is_dir(), "input_dir should be a directory");
    let cargo = dir.join("Cargo.toml");
    let cargo_src = fs::read_to_string(&cargo).expect("read Cargo.toml `[INPUT_DIR]/Cargo.toml`");
    let mut cargo_value = cargo_src.parse::<Value>().unwrap();
    cargo_value
        .as_table_mut()
        .unwrap()
        .get_mut("dependencies")
        .unwrap()
        .as_table_mut()
        .unwrap()
        .insert(
            "buf-min".into(),
            Value::try_from(Dep {
                version: "",
                optional: true,
            })
            .unwrap(),
        );
    let mut features = BTreeMap::new();
    features.insert("bytes-buf", vec!["buf-min"]);
    cargo_value
        .as_table_mut()
        .unwrap()
        .insert("features".into(), Value::from(features));
    let package_name = cargo_value
        .as_table()
        .unwrap()
        .get("package")
        .unwrap()
        .as_table()
        .unwrap()
        .get("name")
        .unwrap()
        .as_str()
        .unwrap();
    let src = dir.join("src");
    let test = dir.join("tests");
    let template = src.join("_lib.rs");
    let template_src =
        fs::read_to_string(&template).expect("read template `[INPUT_DIR]/src/_lib.rs`");
    let pairs = parse_template(&template_src);
    let code = derive(&pairs);
    eprintln!("{}", code);

    // fs::write(&cargo, toml::to_string_pretty(&cargo_value).unwrap()).unwrap();
}

fn parse_template(s: &str) -> Vec<Pair> {
    let mut pairs = syn::parse_str::<Builder>(s)
        .and_then(Builder::build)
        .unwrap();

    // need order for calculate ranges
    pairs.sort_by_key(|p| p.ch);
    // check repeated
    for i in 0..pairs.len() - 1 {
        let p1 = &pairs[i];
        let p2 = &pairs[i + 1];
        assert!(!(p1.ch == p2.ch), "{:?} and {:?} are repeated", p1, p2);
    }

    pairs
}

/// Generate static tables and call macros
fn derive(pairs: &[Pair]) -> TokenStream {
    let code = Generator::new(&pairs).build();
    code.parse().unwrap()
}

#[derive(Debug)]
pub(crate) struct Pair {
    ch: u8,
    quote: String,
}

impl Pair {
    pub fn new<I: Into<String>>(ch: u8, quote: I) -> Self {
        Pair {
            ch,
            quote: quote.into(),
        }
    }
}

struct Ch(u8);

impl Ch {
    fn new(n: u8) -> Result<Self, ()> {
        match n < (i8::MAX as u8) {
            true => Ok(Ch(n)),
            false => Err(()),
        }
    }
}

impl Parse for Ch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::Lit;
        let lit: &Lit = &input.parse()?;
        let map_err = |_: ()| syn::Error::new(lit.span(), "Only accept ASCII characters");
        Ok(match lit {
            Lit::Byte(v) => Ch::new(v.value()).map_err(map_err)?,
            Lit::Char(v) => Ch::new(v.value() as u8).map_err(map_err)?,
            Lit::Int(v) => v
                .base10_parse::<i8>()
                .map_err(|_| ())
                .and_then(|x| Ch::new(x as u8))
                .map_err(map_err)?,
            _ => return Err(map_err(())),
        })
    }
}

struct PairBuilder {
    ch: Ch,
    _s: Token![->],
    quote: syn::LitStr,
}

impl Parse for PairBuilder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(PairBuilder {
            ch: input.parse()?,
            _s: input.parse()?,
            quote: input.parse()?,
        })
    }
}

/// Proc macro arguments parser
struct Builder {
    _path: Ident,
    _bang_token: Bang,
    _delimiter: Paren,
    pairs: Punctuated<PairBuilder, Token![,]>,
    _dots: Token![;],
}

impl Parse for Builder {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> syn::Result<Self> {
        let tokens;
        Ok(Self {
            _path: input.parse()?,
            _bang_token: input.parse()?,
            _delimiter: parenthesized!(tokens in input),
            pairs: Punctuated::parse_separated_nonempty(&tokens)?,
            _dots: input.parse()?,
        })
    }
}

impl Builder {
    /// Consume and return arguments data
    fn build(self) -> syn::Result<Vec<Pair>> {
        let Builder { pairs, .. } = self;

        Ok(pairs
            .into_pairs()
            .map(|x| x.into_value())
            .map(|x| Pair::new(x.ch.0, x.quote.value()))
            .collect())
    }
}

struct Generator<'a> {
    pairs: &'a [Pair],
}

impl<'a> Generator<'a> {
    pub fn new(pairs: &[Pair]) -> Generator {
        Generator { pairs }
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
            write!(buf, "const V_ESCAPE_CHAR: u8 = {};", self.pairs[0].ch);
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
        self.write_ranges(buf);
    }

    fn write_ch(&self, buf: &mut String) {
        let code = if self.pairs.len() == 1 {
            r#"
                mod chars {
                    use super::*;
                    v_escape::escape_char!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                    v_escape::escape_char_ptr!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                    v_escape::escape_char_bytes!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                }
            "#
        } else {
            r#"
                mod chars {
                    use super::*;
                    v_escape::escape_char!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                    v_escape::escape_char_ptr!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                    v_escape::escape_char_bytes!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                }
            "#
        };
        buf.push_str(code);
    }

    fn write_scalar(&self, buf: &mut String) {
        let code = if self.pairs.len() == 1 {
            r#"
                mod scalar {
                    use super::*;
                    v_escape::escape_scalar!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                    v_escape::escape_scalar_ptr!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                    v_escape::escape_scalar_bytes!(one V_ESCAPE_CHAR, V_ESCAPE_QUOTES);
                }
            "#
        } else {
            r#"
                mod scalar {
                    use super::*;
                    v_escape::escape_scalar!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                    v_escape::escape_scalar_ptr!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                    v_escape::escape_scalar_bytes!(V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN);
                }
            "#
        };
        buf.push_str(code);
    }

    fn write_ranges(&self, buf: &mut String) {
        buf.push_str(r#"#[cfg(all(target_arch = "x86_64", not(v_escape_nosimd)))]"#);
        buf.push_str("mod ranges {");

        let ranges = self.calculate_ranges();

        for i in &["avx", "sse"] {
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
            // self.write_macro_tt(buf, ranges);
            buf.push_str(");");
            buf.push_str("v_escape::escape_ranges_ptr!(");
            buf.push_str(i);
            if self.pairs.len() == 1 {
                buf.push_str("2 (V_ESCAPE_CHAR, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            } else {
                buf.push_str("2 (V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            }
            // self.write_macro_tt(buf, ranges);
            buf.push_str(");");
            buf.push_str("v_escape::escape_ranges_bytes!(");
            buf.push_str(i);
            if self.pairs.len() == 1 {
                buf.push_str("2 (V_ESCAPE_CHAR, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            } else {
                buf.push_str("2 (V_ESCAPE_TABLE, V_ESCAPE_QUOTES, V_ESCAPE_LEN) ");
            }
            // self.write_macro_tt(buf, ranges);
            buf.push_str(");");
            buf.push('}');
        }
        buf.push('}');
    }

    fn write_cfg_if(&self, buf: &mut String) {}

    fn write_macro_tt<T, I>(&self, buf: &mut String, i: I)
    where
        T: Display,
        I: IntoIterator<Item = T>,
    {
        for c in i.into_iter() {
            buf.write_fmt(format_args!("{}, ", c)).unwrap();
        }
    }

    fn ch(&self, i: usize) -> u8 {
        self.pairs[i].ch
    }

    fn calculate_ranges(&self) -> Switch {
        use Switch::*;
        assert_ne!(self.pairs.len(), 0);

        if self.pairs.len() == 1 {
            return A { a: self.ch(0) };
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
            0 => Ar {
                la: self.ch(0),
                ra: self.ch(e),
            },
            1 => {
                if e == 1 {
                    AB {
                        a: self.ch(0),
                        b: self.ch(e),
                    }
                } else {
                    let i = d[0].0;
                    if i == 0 {
                        ArB {
                            la: self.ch(i + 1),
                            ra: self.ch(e),
                            b: self.ch(0),
                        }
                    } else if i + 1 != e {
                        ArBr {
                            la: self.ch(0),
                            ra: self.ch(i),
                            lb: self.ch(i + 1),
                            rb: self.ch(e),
                        }
                    } else {
                        ArB {
                            la: self.ch(0),
                            ra: self.ch(i),
                            b: self.ch(i + 1),
                        }
                    }
                }
            }
            _ => {
                if e <= 2 {
                    assert_eq!(e, 2);
                    return ABC {
                        a: self.ch(0),
                        b: self.ch(1),
                        c: self.ch(2),
                    };
                }

                let (d, _) = d.split_at_mut(2);
                d.sort_unstable_by_key(|d| d.0);

                let f = d[0].0;
                let l = d[1].0;

                if f + 1 == l {
                    if f == 0 {
                        self.push_1_ranges_2_equals_at_first(f, l, e)
                    } else if l + 1 == e {
                        self.push_1_ranges_2_equals_at_last(f, l, e)
                    } else {
                        self.push_2_ranges_1_equals(f, l, e)
                    }
                } else if f == 0 {
                    if l + 1 == e {
                        self.push_1_ranges_2_equals_at_first_last(f, l, e)
                    } else {
                        self.push_2_ranges_1_equals_at_first(f, l, e)
                    }
                } else if l + 1 == e {
                    self.push_2_ranges_1_equals_at_last(f, l, e)
                } else {
                    self.push_3_ranges(f, l, e)
                }
            }
        }
    }

    fn push_1_ranges_2_equals_at_first(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBC {
            la: self.ch(l + 1),
            ra: self.ch(e),
            b: self.ch(0),
            c: self.ch(f + 1),
        }
    }

    fn push_1_ranges_2_equals_at_last(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBC {
            la: self.ch(0),
            ra: self.ch(f),
            b: self.ch(l),
            c: self.ch(e),
        }
    }

    fn push_1_ranges_2_equals_at_first_last(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBC {
            la: self.ch(f + 1),
            ra: self.ch(l),
            b: self.ch(0),
            c: self.ch(e),
        }
    }

    fn push_2_ranges_1_equals(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrC {
            la: self.ch(0),
            ra: self.ch(f),
            lb: self.ch(l + 1),
            rb: self.ch(e),
            c: self.ch(f + 1),
        }
    }

    fn push_2_ranges_1_equals_at_first(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrC {
            la: self.ch(f + 1),
            ra: self.ch(l),
            lb: self.ch(l + 1),
            rb: self.ch(e),
            c: self.ch(0),
        }
    }

    fn push_2_ranges_1_equals_at_last(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrC {
            la: self.ch(0),
            ra: self.ch(f),
            lb: self.ch(f + 1),
            rb: self.ch(l),
            c: self.ch(e),
        }
    }

    fn push_3_ranges(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrCr {
            la: self.ch(0),
            ra: self.ch(f),
            lb: self.ch(f + 1),
            rb: self.ch(l),
            lc: self.ch(l + 1),
            rc: self.ch(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Switch::*;

    static E: &str = "f";

    #[test]
    fn test_1_escape() {
        let pairs = &[Pair::new(0, E)];
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), A { a: 0 })
    }

    #[test]
    fn test_2_escape() {
        let pairs = &[Pair::new(0, E), Pair::new(2, E)];
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), AB { a: 0, b: 2 })
    }

    #[test]
    fn test_3_escape() {
        let pairs = &[Pair::new(0, E), Pair::new(2, E), Pair::new(4, E)];
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), ABC { a: 0, b: 2, c: 4 })
    }

    #[test]
    fn test_1_range() {
        let pairs = &[Pair::new(0, E), Pair::new(1, E)];
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![0, 1])
    }

    #[test]
    fn test_2_range() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(3, E),
            Pair::new(4, E),
        ];
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![0, 1, 3, 4])
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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![0, 1, 3, 4, 6, 7]);
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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![0, 9, 50, 64, 126, 127])
    }

    #[test]
    fn test_1_range_1_escape() {
        let pairs = &[Pair::new(0, E), Pair::new(1, E), Pair::new(3, E)];
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![0, 1, 3]);

        let pairs = &[Pair::new(0, E), Pair::new(2, E), Pair::new(3, E)];
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![2, 3, 0]);

        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(2, E),
            Pair::new(4, E),
        ];
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![0, 2, 4]);

        let pairs = &[
            Pair::new(50, E),
            Pair::new(51, E),
            Pair::new(52, E),
            Pair::new(53, E),
            Pair::new(54, E),
            Pair::new(55, E),
            Pair::new(67, E),
        ];
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![50, 55, 67]);
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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![0, 1, 3, 4, 6]);
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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![4, 5, 7, 8, 0]);
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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![14, 16, 50, 52, 98]);
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

        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![14, 16, 50, 52, 98]);
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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![14, 19, 50, 58, 98]);
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

        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![14, 16, 50, 52, 98]);
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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![14, 19, 50, 58, 98]);
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

        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![60, 61, 80, 81, 65]);

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

        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![52, 62, 101, 120, 80]);
    }

    #[test]
    fn test_1_range_2_escape_a() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(1, E),
            Pair::new(4, E),
            Pair::new(6, E),
        ];
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![0, 1, 4, 6, 128]);

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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![0, 14, 73, 127, 128]);
    }

    #[test]
    fn test_1_range_2_escape_b() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(2, E),
            Pair::new(5, E),
            Pair::new(6, E),
        ];
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![5, 6, 0, 2, 128]);

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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![5, 18, 0, 2, 128]);
    }

    #[test]
    fn test_1_range_2_escape_c() {
        let pairs = &[
            Pair::new(0, E),
            Pair::new(2, E),
            Pair::new(3, E),
            Pair::new(8, E),
        ];
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![2, 3, 0, 8, 128]);

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
        let g = Generator::new(pairs);

        // assert_eq!(g.calculate_ranges(), vec![2, 17, 0, 127, 128]);
    }
}
