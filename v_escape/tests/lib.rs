#![allow(dead_code)]
#[macro_use]
extern crate v_escape;

new_escape_sized!(MyEscape, "60->foo");

macro_rules! test {
    ($name:ident, $escapes:expr, $escaped:expr) => {
        use std::borrow::Cow;

        let empty = "";
        let escapes = $escapes;
        let escaped = $escaped;
        let string_long: &str = &"foobar".repeat(1024);
        let string = $escapes.to_string();
        let cow = Cow::Owned($escapes.to_string());

        assert_eq!($name::from(empty).to_string(), empty);
        assert_eq!($name::from(escapes).to_string(), escaped);
        assert_eq!(escape(&cow).to_string(), escaped);
        assert_eq!(escape(&string).to_string(), escaped);
        assert_eq!($name::from(string_long).to_string(), string_long);
        assert_eq!(
            $name::from(escapes.repeat(1024).as_ref()).to_string(),
            escaped.repeat(1024)
        );
        assert_eq!(
            $name::from(
                [string_long, &escapes.repeat(13)]
                    .join("")
                    .repeat(1024)
                    .as_ref()
            )
            .to_string(),
            [string_long, &escaped.repeat(13)].join("").repeat(1024)
        );
    };
}

macro_rules! test_sized {
    ($name:ident, $escapes:expr, $escaped:expr) => {
        let empty = "";
        let escapes = $escapes;
        let escaped = $escaped;
        let string_long: &str = &"foobar".repeat(1024);

        assert_eq!($name::from(empty).size(), empty.len());
        assert_eq!($name::from(escapes).size(), escaped.len());
        assert_eq!($name::from(string_long).size(), string_long.len());
        assert_eq!(
            $name::from(escapes.repeat(1024).as_ref()).size(),
            escaped.repeat(1024).len()
        );
        assert_eq!(
            $name::from(
                [string_long, &escapes.repeat(13)]
                    .join("")
                    .repeat(1024)
                    .as_ref()
            )
            .size(),
            [string_long, &escaped.repeat(13)]
                .join("")
                .repeat(1024)
                .len()
        );
    };
}

#[test]
fn test_escape() {
    test!(MyEscape, "<", "foo");
}

#[test]
fn test_sized() {
    test_sized!(MyEscape, "<", "foo");
}

mod no_simd {
    mod a {
        new_escape!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            simd = false
        );
        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }
    }

    mod b {
        new_escape_sized!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            simd = false
        );
        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<=ABPQ", "bcadef");
        }
    }
}

#[cfg(target_arch = "x86_64")]
mod no_avx {
    mod a {
        new_escape!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            avx = false
        );
        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }
    }

    mod b {
        new_escape_sized!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            avx = false
        );
        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<=ABPQ", "bcadef");
        }
    }
}

mod empty {
    new_escape!(MyE, "65->");

    #[test]
    fn test_escape() {
        test!(MyE, "A", "");
    }
}

#[cfg(target_arch = "x86_64")]
mod test_avx {
    mod a {
        // 3 ranges
        new_escape_sized!(MyE, "65->a || 60->b || 61->c || 66->d || 80->e || 81->f");

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<=ABPQ", "bcadef");
        }
    }

    mod b {
        // 2 ranges and 1 escape
        new_escape_sized!(MyE, "60->a || 61->b || 65->c || 80->d || 81->e");

        #[test]
        fn test_escape() {
            test!(MyE, "<=APQ", "abcde");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<=APQ", "abcde");
        }
    }

    mod c {
        // 1 range and 2 escapes
        new_escape_sized!(MyE, "60->a || 65->c || 80->d || 62->e");

        #[test]
        fn test_escape() {
            test!(MyE, "<>AP", "aecd");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<>AP", "aecd");
        }
    }

    mod d {
        // 3 escapes
        new_escape_sized!(MyE, "60->a || 80->b || 65->c");

        #[test]
        fn test_escape() {
            test!(MyE, "<AP", "acb");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<AP", "acb");
        }
    }

    mod e {
        // 2 ranges
        new_escape_sized!(MyE, "60->a || 61->b || 81->c || 80->d || 62->e");

        #[test]
        fn test_escape() {
            test!(MyE, "<=>PQ", "abedc");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<=>PQ", "abedc");
        }
    }

    mod f {
        // 1 range and 1 escape
        new_escape_sized!(MyE, "60->a || 61->b || 80->c || 62->d");

        #[test]
        fn test_escape() {
            test!(MyE, "<=>P", "abdc");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<=>P", "abdc");
        }
    }

    mod g {
        // 2 escapes
        new_escape_sized!(MyE, "60->a || 80->b");

        #[test]
        fn test_escape() {
            test!(MyE, "<P", "ab");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<P", "ab");
        }
    }

    mod h {
        // 1 range
        new_escape_sized!(MyE, "60->a || 61->b");

        #[test]
        fn test_escape() {
            test!(MyE, "<=", "ab");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<=", "ab");
        }
    }

    mod i {
        // 1 escapes
        new_escape_sized!(MyE, "60->f");

        #[test]
        fn test_escape() {
            test!(MyE, "<", "f");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, "<", "f");
        }
    }
}

mod char_syntax {
    mod a {
        new_escape_sized!(MyE, " ->f");

        #[test]
        fn test_escape() {
            test!(MyE, " ", "f");
        }

        #[test]
        fn test_sized() {
            test_sized!(MyE, " ", "f");
        }
    }
}

#[cfg(target_arch = "x86_64")]
mod simd_loops {
    unsafe fn memchr(n1: u8, bytes: &[u8]) -> Option<usize> {
        use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_set1_epi8};

        let v_a = _mm256_set1_epi8(n1 as i8);

        let len = bytes.len();
        let start_ptr = bytes.as_ptr();
        let mut ptr = start_ptr;

        macro_rules! write_mask {
            ($mask:ident, $ptr:ident) => {{
                return Some(_v_escape_sub!($ptr, start_ptr) + $mask.trailing_zeros() as usize);
            }};
        }

        macro_rules! write_forward {
            ($mask: ident, $align:ident) => {{
                let cur = $mask.trailing_zeros() as usize;

                if cur < $align {
                    return Some(_v_escape_sub!(ptr, start_ptr) + cur);
                }
            }};
        }

        macro_rules! masking {
            ($a:ident) => {{
                _mm256_cmpeq_epi8($a, v_a)
            }};
        }

        loop_m256_128!(len, ptr, start_ptr, bytes);

        None
    }

    unsafe fn memchr4(n0: u8, n1: u8, n2: u8, n3: u8, bytes: &[u8]) -> Option<usize> {
        use std::arch::x86_64::{_mm_cmpestri, _mm_setr_epi8};
        const NEEDLE_LEN: i32 = 4;
        let needle = _mm_setr_epi8(
            n0 as i8, n1 as i8, n2 as i8, n3 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        );

        let len = bytes.len();
        let start_ptr = bytes.as_ptr();
        let mut ptr = start_ptr;

        macro_rules! write_mask {
            ($mask:ident) => {{
                if $mask < 16 {
                    return Some(_v_escape_sub!(ptr, start_ptr) + $mask as usize);
                }
            }};
        }

        macro_rules! masking {
            ($a:ident, $len:ident) => {{
                _mm_cmpestri(needle, NEEDLE_LEN, $a, $len as i32, 0)
            }};
        }

        loop_m128!(len, ptr, start_ptr, bytes);

        None
    }

    mod avx {
        pub unsafe fn memchr4(n0: u8, n1: u8, n2: u8, n3: u8, bytes: &[u8]) -> Option<usize> {
            use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_set1_epi8};

            let v_a = _mm256_set1_epi8(n0 as i8);
            let v_b = _mm256_set1_epi8(n1 as i8);
            let v_c = _mm256_set1_epi8(n2 as i8);
            let v_d = _mm256_set1_epi8(n3 as i8);

            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let mut ptr = start_ptr;

            macro_rules! write_mask {
                ($mask:ident, $ptr:ident) => {{
                    return Some(_v_escape_sub!($ptr, start_ptr) + $mask.trailing_zeros() as usize);
                }};
            }

            macro_rules! write_forward {
                ($mask: ident, $align:ident) => {{
                    let cur = $mask.trailing_zeros() as usize;

                    if cur < $align {
                        return Some(_v_escape_sub!(ptr, start_ptr) + cur);
                    }
                }};
            }

            macro_rules! masking {
                ($a:ident) => {{
                    _mm256_or_si256(
                        _mm256_or_si256(_mm256_cmpeq_epi8($a, v_a), _mm256_cmpeq_epi8($a, v_b)),
                        _mm256_or_si256(_mm256_cmpeq_epi8($a, v_c), _mm256_cmpeq_epi8($a, v_d)),
                    )
                }};
            }

            loop_m256_64!(len, ptr, start_ptr, bytes);

            None
        }
    }

    #[test]
    fn test_loops() {
        assert_eq!(unsafe { memchr(b'a', b"b") }, None);
        assert_eq!(unsafe { memchr(b'a', b"ba") }, Some(1));
        assert_eq!(
            unsafe { memchr(b'a', &[&"b".repeat(1024), "a"].join("").as_bytes()) },
            Some(1024)
        );
        assert_eq!(unsafe { memchr4(b'a', b'b', b'c', b'd', b"e") }, None);
        assert_eq!(unsafe { memchr4(b'a', b'b', b'c', b'd', b"abcd") }, Some(0));
        assert_eq!(
            unsafe {
                memchr4(
                    b'a',
                    b'b',
                    b'c',
                    b'd',
                    &[&"e".repeat(1024), "b"].join("").as_bytes(),
                )
            },
            Some(1024)
        );
        assert_eq!(unsafe { avx::memchr4(b'a', b'b', b'c', b'd', b"e") }, None);
        assert_eq!(
            unsafe { avx::memchr4(b'a', b'b', b'c', b'd', b"abcd") },
            Some(0)
        );
        assert_eq!(
            unsafe {
                avx::memchr4(
                    b'a',
                    b'b',
                    b'c',
                    b'd',
                    &[&"e".repeat(1024), "b"].join("").as_bytes(),
                )
            },
            Some(1024)
        );
    }
}
