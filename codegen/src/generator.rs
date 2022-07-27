use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::str;

use crate::functions::escape_new;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde::Serialize;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bang, Paren};
use syn::Lit;
use syn::{parenthesized, Token};
use toml::Value;

use crate::build::build_file;
use crate::macros::write_bytes;
use crate::ranges::{escape_range, escape_range_bytes, Feature, Switch};
use crate::scalar::{escape_scalar, escape_scalar_bytes, ArgScalar};
use crate::tests::build_tests;
use crate::utils::ident;

#[derive(Serialize)]
struct Dep {
    version: &'static str,
    optional: bool,
}

pub fn generate<P: AsRef<Path>>(dir: P) {
    // TODO: add documentation
    let head =
        "//! autogenerated by v_escape_codegen@".to_string() + env!("CARGO_PKG_VERSION") + "\n";
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
                version: "^0.6.1",
                optional: true,
            })
            .unwrap(),
        );
    let doc: Value = toml::from_str(
        r"
    [docs.rs]
    all-features = true
    ",
    )
    .unwrap();
    cargo_value
        .as_table_mut()
        .unwrap()
        .get_mut("package")
        .unwrap()
        .as_table_mut()
        .unwrap()
        .insert("metadata".to_string(), doc);

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
    if !test.exists() {
        fs::create_dir(&test).unwrap();
    }
    let template = src.join("_lib.rs");
    let template_src =
        fs::read_to_string(&template).expect("read template `[INPUT_DIR]/src/_lib.rs`");
    let pairs = parse_template(&template_src);
    let mut code = Generator::new(&pairs).build();
    let name = heck::AsPascalCase(package_name).to_string();
    let struct_name = &ident(&name);
    let name = &ident(package_name);
    code.extend(escape_new(struct_name));
    let code_pretty = prettyplease::unparse(&syn::parse2(code).unwrap());
    let escapes = String::from_utf8(pairs.iter().map(|x| x.ch).collect::<Vec<u8>>()).unwrap();
    let escaped = pairs
        .iter()
        .map(|x| x.quote.as_str())
        .collect::<Vec<&str>>()
        .join("");
    let code_test = build_tests(name, struct_name, escapes, escaped);
    let code_test_pretty = prettyplease::unparse(&syn::parse2(code_test).unwrap());
    let code_build = build_file();
    let code_build_pretty = prettyplease::unparse(&syn::parse2(code_build).unwrap());

    fs::write(&cargo, toml::to_string_pretty(&cargo_value).unwrap()).unwrap();
    fs::write(src.join("lib.rs"), head.clone() + &code_pretty).unwrap();
    fs::write(test.join("lib.rs"), head.clone() + &code_test_pretty).unwrap();
    fs::write(dir.join("build.rs"), head + &code_build_pretty).unwrap();
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
        assert_ne!(p1.ch, p2.ch, "{:?} and {:?} are repeated", p1, p2);
    }

    pairs
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

pub type Tables = (Ident, Ident, Ident);
struct Generator<'a> {
    pairs: &'a [Pair],
}

impl<'a> Generator<'a> {
    pub fn new(pairs: &[Pair]) -> Generator {
        Generator { pairs }
    }

    pub fn build(&self) -> TokenStream {
        let mut buf = TokenStream::new();

        let tables = &self.write_static_table(&mut buf);
        self.write_utils(&mut buf);
        self.write_functions(tables, &mut buf);

        buf
    }

    fn write_utils(&self, buf: &mut TokenStream) {
        buf.extend(quote! {
            #[inline(always)]
            fn sub(a: *const u8, b: *const u8) -> usize {
                debug_assert!(b <= a);
                (a as usize) - (b as usize)
            }
        })
    }

    fn write_static_table(&self, buf: &mut TokenStream) -> Tables {
        let len = self.pairs.len();
        let v_char = ident("V_ESCAPE_CHARS");
        let v_quotes = ident("V_ESCAPE_QUOTES");
        let v_len = ident("V_ESCAPE_LEN");

        if len == 1 {
            let ch = self.pairs[0].ch;
            let quote = &self.pairs[0].quote;
            buf.extend(quote! {
                const #v_char: u8 = #ch;
                static #v_quotes: &str = #quote;
            })
        } else {
            let mut chs = Vec::with_capacity(256);
            for i in 0..=255_u8 {
                let n = self.pairs.binary_search_by(|s| s.ch.cmp(&i)).unwrap_or(len);
                chs.push(n as u8)
            }
            let quotes: Vec<&str> = self.pairs.iter().map(|s| s.quote.as_str()).collect();
            buf.extend(quote! {
                static #v_char: [u8; 256] = [#(#chs),*];
                static #v_quotes: [&str; #len] = [#(#quotes),*];
            })
        }
        buf.extend(quote! {
            const #v_len: usize = #len;
        });

        (v_char, v_quotes, v_len)
    }

    fn write_functions(&self, tables: &Tables, buf: &mut TokenStream) {
        let switch = self.calculate_ranges();
        self.write_scalar(switch, tables, buf);
        self.write_ranges(switch, tables, buf)
    }

    fn write_scalar(&self, s: Switch, table: &Tables, buf: &mut TokenStream) {
        let ptr = &ident("ptr");
        let start_ptr = &ident("start_ptr");
        let end_ptr = &ident("end_ptr");
        let bytes = &ident("bytes");
        let fmt = &ident("fmt");
        let len = &ident("len");
        let start = &ident("start");
        let body = escape_scalar(
            ArgScalar {
                ptr,
                end_ptr,
                bytes,
                fmt,
                start_ptr,
                start,
                s,
            },
            table,
        );

        let body_bytes = escape_scalar_bytes(
            ArgScalar {
                ptr,
                end_ptr,
                bytes,
                fmt,
                start_ptr,
                start,
                s,
            },
            table,
        );

        let write_1 = write_bytes(&quote! { &#bytes[#start..] }, fmt);
        buf.extend(quote! {
            pub mod scalar {
                use super::*;
                pub struct __Escaped<'a>(&'a[u8]);
                impl<'a> std::fmt::Display for __Escaped<'a> {
                    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                        _escape(self.0, fmt)
                    }
                }
                pub fn escape(s: &str) -> __Escaped {
                    __Escaped(s.as_bytes())
                }
                pub fn _escape(#bytes: &[u8], #fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    unsafe {
                        let #len = #bytes.len();
                        let #start_ptr = #bytes.as_ptr();
                        let #end_ptr = #bytes[len..].as_ptr();
                        let mut #ptr = #start_ptr;
                        let mut #start = 0;
                        #body
                        debug_assert!(start <= len);
                        if start < len {
                            fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..len]))?;
                        }
                        Ok(())
                    }
                }

                #[cfg(feature = "bytes-buf")]
                pub fn b_escape<B: buf_min::Buffer>(#bytes: &[u8], #fmt: &mut B) {
                    unsafe {
                        let #len = #bytes.len();
                        let #start_ptr = #bytes.as_ptr();
                        let #end_ptr = #bytes[len..].as_ptr();
                        let mut #ptr = #start_ptr;
                        let mut #start = 0;
                        #body_bytes
                        debug_assert!(#start <= #len);
                        if #start < #len {
                            #write_1
                        }
                    }
                }
            }
        });
    }

    fn write_ranges(&self, switch: Switch, tables: &Tables, buf: &mut TokenStream) {
        let mod_avx = escape_range(switch, tables, Feature::Avx2);
        let mod_sse = escape_range(switch, tables, Feature::Sse2);
        let mod_avx_bytes = escape_range_bytes(switch, tables, Feature::Avx2);
        let mod_sse_bytes = escape_range_bytes(switch, tables, Feature::Sse2);

        buf.extend(quote! {
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            pub mod ranges {
                pub mod avx {
                    use super::super::*;
                    #mod_avx
                    #mod_avx_bytes
                }
                pub mod sse {
                    use super::super::*;
                    #mod_sse
                    #mod_sse_bytes
                }
            }
        })
    }

    #[inline]
    fn ch(&self, i: usize) -> i8 {
        self.pairs[i].ch as i8
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

    #[inline]
    fn push_1_ranges_2_equals_at_first(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBC {
            la: self.ch(l + 1),
            ra: self.ch(e),
            b: self.ch(0),
            c: self.ch(f + 1),
        }
    }

    #[inline]
    fn push_1_ranges_2_equals_at_last(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBC {
            la: self.ch(0),
            ra: self.ch(f),
            b: self.ch(l),
            c: self.ch(e),
        }
    }

    #[inline]
    fn push_1_ranges_2_equals_at_first_last(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBC {
            la: self.ch(f + 1),
            ra: self.ch(l),
            b: self.ch(0),
            c: self.ch(e),
        }
    }

    #[inline]
    fn push_2_ranges_1_equals(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrC {
            la: self.ch(0),
            ra: self.ch(f),
            lb: self.ch(l + 1),
            rb: self.ch(e),
            c: self.ch(f + 1),
        }
    }

    #[inline]
    fn push_2_ranges_1_equals_at_first(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrC {
            la: self.ch(f + 1),
            ra: self.ch(l),
            lb: self.ch(l + 1),
            rb: self.ch(e),
            c: self.ch(0),
        }
    }

    #[inline]
    fn push_2_ranges_1_equals_at_last(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrC {
            la: self.ch(0),
            ra: self.ch(f),
            lb: self.ch(f + 1),
            rb: self.ch(l),
            c: self.ch(e),
        }
    }

    #[inline]
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
    macro_rules! pair {
        ($($l:literal),*) => { &[$(Pair::new($l, E)),*] };
    }

    #[test]
    fn test_1_escape() {
        let pairs = pair!(0);
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), A { a: 0 })
    }

    #[test]
    fn test_2_escape() {
        let pairs = pair!(0, 2);
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), AB { a: 0, b: 2 })
    }

    #[test]
    fn test_3_escape() {
        let pairs = pair!(0, 2, 4);
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), ABC { a: 0, b: 2, c: 4 })
    }

    #[test]
    fn test_1_range() {
        let pairs = pair!(0, 1);
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), Ar { la: 0, ra: 1 })
    }

    #[test]
    fn test_2_range() {
        let pairs = pair!(0, 1, 3, 4);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBr {
                la: 0,
                ra: 1,
                lb: 3,
                rb: 4
            }
        )
    }

    #[test]
    fn test_3_range() {
        let pairs = pair!(0, 1, 3, 4, 6, 7);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrCr {
                la: 0,
                ra: 1,
                lb: 3,
                rb: 4,
                lc: 6,
                rc: 7
            }
        );

        let pairs = pair!(0, 1, 3, 5, 7, 9, 50, 52, 55, 60, 61, 62, 63, 64, 126, 127);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrCr {
                la: 0,
                ra: 9,
                lb: 50,
                rb: 64,
                lc: 126,
                rc: 127
            }
        )
    }

    #[test]
    fn test_1_range_1_escape() {
        let pairs = pair!(0, 1, 3);
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), ArB { la: 0, ra: 1, b: 3 });

        let pairs = pair!(0, 2, 3);
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), ArB { la: 2, ra: 3, b: 0 });

        let pairs = pair!(0, 1, 2, 4);
        let g = Generator::new(pairs);

        assert_eq!(g.calculate_ranges(), ArB { la: 0, ra: 2, b: 4 });

        let pairs = pair!(50, 51, 52, 53, 54, 55, 67);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArB {
                la: 50,
                ra: 55,
                b: 67
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_a() {
        let pairs = pair!(0, 1, 3, 4, 6);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrC {
                la: 0,
                ra: 1,
                lb: 3,
                rb: 4,
                c: 6
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_b() {
        let pairs = pair!(0, 4, 5, 7, 8);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrC {
                la: 4,
                ra: 5,
                lb: 7,
                rb: 8,
                c: 0
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_c() {
        let pairs = pair!(14, 15, 16, 50, 51, 52, 98);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrC {
                la: 14,
                ra: 16,
                lb: 50,
                rb: 52,
                c: 98
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_d() {
        let pairs = pair!(14, 15, 16, 50, 51, 52, 98);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrC {
                la: 14,
                ra: 16,
                lb: 50,
                rb: 52,
                c: 98
            }
        );
        let pairs = pair!(14, 15, 16, 17, 18, 19, 50, 51, 52, 53, 54, 55, 56, 57, 58, 98);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrC {
                la: 14,
                ra: 19,
                lb: 50,
                rb: 58,
                c: 98
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_e() {
        let pairs = pair!(14, 16, 50, 51, 52, 98);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrC {
                la: 14,
                ra: 16,
                lb: 50,
                rb: 52,
                c: 98
            }
        );

        let pairs = pair!(14, 16, 17, 18, 19, 50, 52, 53, 56, 57, 58, 98);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrC {
                la: 14,
                ra: 19,
                lb: 50,
                rb: 58,
                c: 98
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_f() {
        let pairs = pair!(60, 61, 65, 80, 81);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrC {
                la: 60,
                ra: 61,
                lb: 80,
                rb: 81,
                c: 65
            }
        );

        let pairs = pair!(52, 53, 56, 58, 60, 61, 62, 80, 101, 102, 104, 105, 108, 110, 120);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBrC {
                la: 52,
                ra: 62,
                lb: 101,
                rb: 120,
                c: 80
            }
        );
    }

    #[test]
    fn test_1_range_2_escape_a() {
        let pairs = pair!(0, 1, 4, 6);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBC {
                la: 0,
                ra: 1,
                b: 4,
                c: 6
            }
        );

        let pairs = pair!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 73, 127);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBC {
                la: 0,
                ra: 14,
                b: 73,
                c: 127
            }
        );
    }

    #[test]
    fn test_1_range_2_escape_b() {
        let pairs = pair!(0, 2, 5, 6);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBC {
                la: 5,
                ra: 6,
                b: 0,
                c: 2
            }
        );

        let pairs = pair!(0, 2, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBC {
                la: 5,
                ra: 18,
                b: 0,
                c: 2
            }
        );
    }

    #[test]
    fn test_1_range_2_escape_c() {
        let pairs = pair!(0, 2, 3, 8);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBC {
                la: 2,
                ra: 3,
                b: 0,
                c: 8
            }
        );

        let pairs = pair!(0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 127);
        let g = Generator::new(pairs);

        assert_eq!(
            g.calculate_ranges(),
            ArBC {
                la: 2,
                ra: 17,
                b: 0,
                c: 127
            }
        );
    }
}
