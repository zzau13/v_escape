use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn all_utf8_less() -> TokenStream {
    quote! {
        #![allow(unused)]
        fn all_utf8_less(less: &str) -> String {
            use std::char::from_u32;
            assert_eq!(less.len(), less.as_bytes().len());

            let less = less.as_bytes();
            let mut buf = String::with_capacity(204_672 - less.len());

            for i in 0..0x80u8 {
                if !less.contains(&i) {
                    buf.push(from_u32(i as u32).unwrap())
                }
            }
            for i in 0x80..0xD800 {
                buf.push(from_u32(i).unwrap());
            }
            for i in 0xE000..0x11000 {
                buf.push(from_u32(i).unwrap());
            }

            buf
        }
    }
}

fn result_bytes() -> TokenStream {
    quote! {
        fn result(haystack: &str) -> String {
            let mut buf = Vec::new();
            escape_bytes(haystack, &mut buf);
            String::from_utf8(buf).unwrap()
        }
    }
}

fn result_string() -> TokenStream {
    quote! {
        fn result(haystack: &str) -> String {
            let mut buf = String::new();
            escape_string(haystack, &mut buf);
            buf
        }
    }
}

fn result_fmt() -> TokenStream {
    quote! {
        fn result(haystack: &str) -> String {
            escape_fmt(haystack).to_string()
        }
    }
}

fn tests(escapes: &str, escaped: &str) -> TokenStream {
    quote! {
        #[test]
        fn tests() {
            use std::borrow::Cow;

            let empty = "";
            let escapes = #escapes;
            let escaped = #escaped;
            let utf8: &str = &all_utf8_less(#escapes);
            let empty_heap = String::new();
            let short = "foobar";
            let string_long: &str = &short.repeat(1024);
            let string = #escapes;
            let cow = Cow::Owned(#escapes.to_string());

            assert_eq!(
                result(&[short, escapes, short].join("")),
                [short, escaped, short].join("")
            );

            assert_eq!(result(empty), empty);
            assert_eq!(result(escapes), escaped);
            assert_eq!(result(&empty_heap), empty);
            assert_eq!(result(&cow), escaped);
            assert_eq!(result(&string), escaped);
            assert_eq!(result(&utf8), utf8);
            assert_eq!(result(string_long), string_long);
            assert_eq!(
                result(escapes.repeat(1024).as_ref()),
                escaped.repeat(1024)
            );
            assert_eq!(
                result([short, escapes, short].join("").as_ref()),
                [short, escaped, short].join("")
            );
            assert_eq!(
                result([escapes, short].join("").as_ref()),
                [escaped, short].join("")
            );
            assert_eq!(
                result(["f", escapes, short].join("").as_ref()),
                ["f", escaped, short].join("")
            );
            assert_eq!(
                result(["f", escapes].join("").as_ref()),
                ["f", escaped].join("")
            );
            assert_eq!(
                result(["do", escapes].join("").as_ref()),
                ["do", escaped].join("")
            );
            assert_eq!(
                result(["do", escapes, "b"].join("").as_ref()),
                ["do", escaped, "b"].join("")
            );
            assert_eq!(
                result(escapes.repeat(2).as_ref()),
                escaped.repeat(2)
            );
            assert_eq!(
                result(escapes.repeat(3).as_ref()),
                escaped.repeat(3)
            );
            assert_eq!(
                result(["f", &escapes.repeat(2)].join("").as_ref()),
                ["f", &escaped.repeat(2)].join("")
            );
            assert_eq!(
                result(["do", &escapes.repeat(2)].join("").as_ref()),
                ["do", &escaped.repeat(2)].join("")
            );
            assert_eq!(
                result(["do", &escapes.repeat(2), "bar"].join("").as_ref()),
                ["do", &escaped.repeat(2), "bar"].join("")
            );
            assert_eq!(
                result(["do", &escapes.repeat(3), "bar"].join("").as_ref()),
                ["do", &escaped.repeat(3), "bar"].join("")
            );
            assert_eq!(
                result([&escapes.repeat(3), "bar"].join("").as_ref()),
                [&escaped.repeat(3), "bar"].join("")
            );
            assert_eq!(
                result([short, &escapes.repeat(3), "bar"].join("").as_ref()),
                [short, &escaped.repeat(3), "bar"].join("")
            );
            assert_eq!(
                result([short, &escapes.repeat(5), "bar"].join("").as_ref()),
                [short, &escaped.repeat(5), "bar"].join("")
            );
            assert_eq!(
                result([string_long, &escapes.repeat(13)].join("").repeat(1024).as_ref()),
                [string_long, &escaped.repeat(13)].join("").repeat(1024)
            );
            assert_eq!(
                result([utf8, escapes, short].join("").as_ref()),
                [utf8, escaped, short].join("")
            );
            assert_eq!(
                result([utf8, escapes, utf8].join("").as_ref()),
                [utf8, escaped, utf8].join("")
            );
            assert_eq!(
                result([&utf8.repeat(124), escapes, utf8].join("").as_ref()),
                [&utf8.repeat(124), escaped, utf8].join("")
            );
            assert_eq!(
                result([escapes, &utf8.repeat(124), escapes, utf8].join("").as_ref()),
                [escaped, &utf8.repeat(124), escaped, utf8].join("")
            );
            assert_eq!(
                result([escapes, &utf8.repeat(124), escapes, utf8, escapes].join("").as_ref()),
                [escaped, &utf8.repeat(124), escaped, utf8, escaped].join("")
            );
        }
    }
}

pub fn build_tests(package: &Ident, escapes: &str, escaped: &str) -> TokenStream {
    let all_utf8_less = all_utf8_less();
    let tests = tests(escapes, escaped);
    let result_string = result_string();
    let result_fmt = result_fmt();
    let result_bytes = result_bytes();
    quote! {
        #all_utf8_less
        #[cfg(feature = "string")]
        mod string {
            use super::*;
            use #package::escape_string;
            #result_string
            #tests
        }
        #[cfg(feature = "fmt")]
        mod fmt {
            use super::*;
            use #package::escape_fmt;
            #result_fmt
            #tests
        }
        #[cfg(feature = "bytes")]
        mod bytes {
            use super::*;
            use #package::escape_bytes;
            #result_bytes
            #tests
        }
    }
}
