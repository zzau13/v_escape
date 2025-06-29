#![cfg(all(feature = "string", feature = "fmt"))]
use v_escape_base::{Escapes, EscapesBuilder, Vector, escape_builder};

mod no_false_positive {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct Equal<V: Vector> {
        a: V,
    }

    struct Builder;
    impl EscapesBuilder for Builder {
        type Escapes<V: Vector> = Equal<V>;

        fn new<V: Vector>() -> Self::Escapes<V> {
            Equal { a: V::splat(b'a') }
        }
    }

    impl<V: Vector> Escapes for Equal<V> {
        const ESCAPE_LEN: usize = 1;

        const FALSE_POSITIVE: bool = false;

        type Vector = V;

        #[inline(always)]
        fn masking(&self, vector2: V) -> V {
            self.a.cmpeq(vector2)
        }

        #[inline(always)]
        fn escape(_: usize) -> &'static str {
            "foo"
        }

        #[inline(always)]
        fn position(_: u8) -> usize {
            0
        }

        #[inline(always)]
        fn byte_byte_compare(c: u8) -> bool {
            c == b'a'
        }
    }

    escape_builder!(Builder);

    #[test]
    fn test_escape_bytes() {
        let mut buffer = String::new();
        let haystack = "a".repeat(64);
        escape_string(&haystack, &mut buffer);
        assert_eq!(buffer, "foo".repeat(64));
    }

    #[test]
    fn test_escape_fmt() {
        let haystack = "a".repeat(64);
        let result = escape_fmt(&haystack).to_string();
        assert_eq!(result, "foo".repeat(64));
    }

    // Test empty string
    #[test]
    fn test_empty_string() {
        let mut buffer = String::new();
        escape_string("", &mut buffer);
        assert_eq!(buffer, "");

        let result = escape_fmt("").to_string();
        assert_eq!(result, "");
    }

    // Test string with no escapes
    #[test]
    fn test_no_escapes() {
        let mut buffer = String::new();
        let haystack = "hello world";
        escape_string(haystack, &mut buffer);
        assert_eq!(buffer, haystack);

        let result = escape_fmt(haystack).to_string();
        assert_eq!(result, haystack);
    }

    // Test single character
    #[test]
    fn test_single_character() {
        let mut buffer = String::new();
        escape_string("a", &mut buffer);
        assert_eq!(buffer, "foo");

        let result = escape_fmt("a").to_string();
        assert_eq!(result, "foo");
    }

    // Test mixed content
    #[test]
    fn test_mixed_content() {
        let mut buffer = String::new();
        let haystack = "hello a world a test";
        escape_string(haystack, &mut buffer);
        assert_eq!(buffer, "hello foo world foo test");

        let result = escape_fmt(haystack).to_string();
        assert_eq!(result, "hello foo world foo test");
    }

    // Test consecutive escapes
    #[test]
    fn test_consecutive_escapes() {
        let mut buffer = String::new();
        let haystack = "aaa";
        escape_string(haystack, &mut buffer);
        assert_eq!(buffer, "foofoofoo");

        let result = escape_fmt(haystack).to_string();
        assert_eq!(result, "foofoofoo");
    }

    // Test escape at beginning
    #[test]
    fn test_escape_at_beginning() {
        let mut buffer = String::new();
        let haystack = "ahello";
        escape_string(haystack, &mut buffer);
        assert_eq!(buffer, "foohello");

        let result = escape_fmt(haystack).to_string();
        assert_eq!(result, "foohello");
    }

    // Test escape at end
    #[test]
    fn test_escape_at_end() {
        let mut buffer = String::new();
        let haystack = "helloa";
        escape_string(haystack, &mut buffer);
        assert_eq!(buffer, "hellofoo");

        let result = escape_fmt(haystack).to_string();
        assert_eq!(result, "hellofoo");
    }

    // Test large string
    #[test]
    fn test_large_string() {
        let mut buffer = String::new();
        let haystack = "a".repeat(1000);
        escape_string(&haystack, &mut buffer);
        assert_eq!(buffer, "foo".repeat(1000));

        let result = escape_fmt(&haystack).to_string();
        assert_eq!(result, "foo".repeat(1000));
    }

    // Test with Cow types
    #[test]
    fn test_cow_types() {
        use std::borrow::Cow;

        let mut buffer = String::new();
        let cow_owned = Cow::Owned("a".to_string());
        escape_string(&cow_owned, &mut buffer);
        assert_eq!(buffer, "foo");

        let result = escape_fmt(&cow_owned).to_string();
        assert_eq!(result, "foo");

        let cow_borrowed = Cow::Borrowed("a");
        let mut buffer2 = String::new();
        escape_string(&cow_borrowed, &mut buffer2);
        assert_eq!(buffer2, "foo");

        let result2 = escape_fmt(&cow_borrowed).to_string();
        assert_eq!(result2, "foo");
    }

    // Test byte-by-byte escape functionality
    #[test]
    fn test_byte_byte_escape() {
        let mut buffer = String::new();
        let writer = |s: &str| {
            buffer.push_str(s);
            Ok::<(), ()>(())
        };

        let result = Equal::<()>::byte_byte_escape("a", writer);
        assert!(result.is_ok());
        assert_eq!(buffer, "foo");
    }

    // Test position and escape functions
    #[test]
    fn test_position_and_escape() {
        assert_eq!(Equal::<()>::position(b'a'), 0);
        assert_eq!(Equal::<()>::escape(0), "foo");
    }

    // Test byte_byte_compare
    #[test]
    fn test_byte_byte_compare() {
        assert!(Equal::<()>::byte_byte_compare(b'a'));
        assert!(!Equal::<()>::byte_byte_compare(b'b'));
        assert!(!Equal::<()>::byte_byte_compare(b'z'));
    }

    // Test with different string types
    #[test]
    fn test_different_string_types() {
        let mut buffer = String::new();

        // String
        let string = String::from("a");
        escape_string(&string, &mut buffer);
        assert_eq!(buffer, "foo");

        // &str
        let mut buffer2 = String::new();
        let str_ref = "a";
        escape_string(str_ref, &mut buffer2);
        assert_eq!(buffer2, "foo");

        // Box<str>
        let mut buffer3 = String::new();
        let boxed_str = "a".to_string().into_boxed_str();
        escape_string(&boxed_str, &mut buffer3);
        assert_eq!(buffer3, "foo");
    }

    // Test Display trait implementation
    #[test]
    fn test_display_trait() {
        use std::fmt::Write;

        let haystack = "a";
        let display = escape_fmt(haystack);

        let mut buffer = String::new();
        write!(&mut buffer, "{}", display).unwrap();
        assert_eq!(buffer, "foo");
    }

    // Test with unicode characters (non-ASCII)
    #[test]
    fn test_unicode_characters() {
        let mut buffer = String::new();
        let haystack = "a🚀a";
        escape_string(haystack, &mut buffer);
        assert_eq!(buffer, "foo🚀foo");

        let result = escape_fmt(haystack).to_string();
        assert_eq!(result, "foo🚀foo");
    }

    // Test with emoji and special characters
    #[test]
    fn test_emoji_and_special_chars() {
        let mut buffer = String::new();
        let haystack = "a🎉🌟a";
        escape_string(haystack, &mut buffer);
        assert_eq!(buffer, "foo🎉🌟foo");

        let result = escape_fmt(haystack).to_string();
        assert_eq!(result, "foo🎉🌟foo");
    }

    // Test with very long strings to test vectorization
    #[test]
    fn test_very_long_strings() {
        let mut buffer = String::new();
        let haystack = "a".repeat(10000);
        escape_string(&haystack, &mut buffer);
        assert_eq!(buffer, "foo".repeat(10000));

        let result = escape_fmt(&haystack).to_string();
        assert_eq!(result, "foo".repeat(10000));
    }

    // Test with strings that are exactly vector size
    #[test]
    fn test_vector_sized_strings() {
        // Test with different sizes that might trigger different vectorization paths
        for size in [16, 32, 64, 128] {
            let mut buffer = String::new();
            let haystack = "a".repeat(size);
            escape_string(&haystack, &mut buffer);
            assert_eq!(buffer, "foo".repeat(size));

            let result = escape_fmt(&haystack).to_string();
            assert_eq!(result, "foo".repeat(size));
        }
    }

    // Test with strings that are smaller than vector size
    #[test]
    fn test_small_strings() {
        for size in 1..16 {
            let mut buffer = String::new();
            let haystack = "a".repeat(size);
            escape_string(&haystack, &mut buffer);
            assert_eq!(buffer, "foo".repeat(size));

            let result = escape_fmt(&haystack).to_string();
            assert_eq!(result, "foo".repeat(size));
        }
    }

    // Test with strings that have escapes at specific positions
    #[test]
    fn test_escapes_at_specific_positions() {
        let test_cases = vec![
            ("a", "foo"),
            ("aa", "foofoo"),
            ("aaa", "foofoofoo"),
            ("a a", "foo foo"),
            (" a ", " foo "),
            ("a\na", "foo\nfoo"),
            ("a\ta", "foo\tfoo"),
        ];

        for (input, expected) in test_cases {
            let mut buffer = String::new();
            escape_string(input, &mut buffer);
            assert_eq!(buffer, expected, "Failed for input: {:?}", input);

            let result = escape_fmt(input).to_string();
            assert_eq!(result, expected, "Failed for input: {:?}", input);
        }
    }

    // Test with strings containing only non-escape characters
    #[test]
    fn test_only_non_escape_chars() {
        let test_strings = vec!["hello", "world", "test", "12345", "!@#$%", "🚀🌟🎉"];

        for test_str in test_strings {
            let mut buffer = String::new();
            escape_string(test_str, &mut buffer);
            assert_eq!(buffer, test_str, "Failed for input: {:?}", test_str);

            let result = escape_fmt(test_str).to_string();
            assert_eq!(result, test_str, "Failed for input: {:?}", test_str);
        }
    }

    // Test with strings containing only escape characters
    #[test]
    fn test_only_escape_chars() {
        let mut buffer = String::new();
        let haystack = "a".repeat(10);
        escape_string(&haystack, &mut buffer);
        assert_eq!(buffer, "foo".repeat(10));

        let result = escape_fmt(&haystack).to_string();
        assert_eq!(result, "foo".repeat(10));
    }

    // Test with strings that have escapes at the boundary of vector operations
    #[test]
    fn test_boundary_escapes() {
        // Test with strings that might trigger different vectorization paths
        let sizes = [15, 16, 17, 31, 32, 33, 63, 64, 65];

        for size in sizes {
            let mut buffer = String::new();
            let haystack = "a".repeat(size);
            escape_string(&haystack, &mut buffer);
            assert_eq!(buffer, "foo".repeat(size), "Failed for size: {}", size);

            let result = escape_fmt(&haystack).to_string();
            assert_eq!(result, "foo".repeat(size), "Failed for size: {}", size);
        }
    }

    // Test with strings that have escapes at the very end
    #[test]
    fn test_escapes_at_end() {
        let test_cases = vec![
            ("a", "foo"),
            ("ba", "bfoo"),
            ("cba", "cbfoo"),
            ("dcba", "dcbfoo"),
        ];

        for (input, expected) in test_cases {
            let mut buffer = String::new();
            escape_string(input, &mut buffer);
            assert_eq!(buffer, expected, "Failed for input: {:?}", input);

            let result = escape_fmt(input).to_string();
            assert_eq!(result, expected, "Failed for input: {:?}", input);
        }
    }

    // Test with strings that have escapes at the very beginning
    #[test]
    fn test_escapes_at_beginning() {
        let test_cases = vec![
            ("a", "foo"),
            ("ab", "foob"),
            ("abc", "foobc"),
            ("abcd", "foobcd"),
        ];

        for (input, expected) in test_cases {
            let mut buffer = String::new();
            escape_string(input, &mut buffer);
            assert_eq!(buffer, expected, "Failed for input: {:?}", input);

            let result = escape_fmt(input).to_string();
            assert_eq!(result, expected, "Failed for input: {:?}", input);
        }
    }

    // Test with strings that have escapes in the middle
    #[test]
    fn test_escapes_in_middle() {
        let test_cases = vec![
            ("ba", "bfoo"),
            ("cba", "cbfoo"),
            ("dcba", "dcbfoo"),
            ("edcba", "edcbfoo"),
        ];

        for (input, expected) in test_cases {
            let mut buffer = String::new();
            escape_string(input, &mut buffer);
            assert_eq!(buffer, expected, "Failed for input: {:?}", input);

            let result = escape_fmt(input).to_string();
            assert_eq!(result, expected, "Failed for input: {:?}", input);
        }
    }

    // Test with strings that have multiple escapes in various positions
    #[test]
    fn test_multiple_escapes_various_positions() {
        let test_cases = vec![
            ("aa", "foofoo"),
            ("aaa", "foofoofoo"),
            ("a a", "foo foo"),
            ("a a a", "foo foo foo"),
            (" a a ", " foo foo "),
            ("a\na\na", "foo\nfoo\nfoo"),
        ];

        for (input, expected) in test_cases {
            let mut buffer = String::new();
            escape_string(input, &mut buffer);
            assert_eq!(buffer, expected, "Failed for input: {:?}", input);

            let result = escape_fmt(input).to_string();
            assert_eq!(result, expected, "Failed for input: {:?}", input);
        }
    }
}

mod false_positive {
    use super::*;

    static V_ESCAPE_CHARS: [u8; 256] = [
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 0u8, 6u8,
        6u8, 6u8, 1u8, 2u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 3u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 4u8, 6u8, 5u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8,
        6u8, 6u8, 6u8, 6u8,
    ];
    static V_ESCAPE_QUOTES: [&str; 6usize] =
        ["&quot;", "&amp;", "&#x27;", "&#x2f;", "&lt;", "&gt;"];
    const V_ESCAPE_LEN: usize = 6usize;

    #[derive(Debug, Clone, Copy)]
    struct Escape<V: Vector> {
        translation_a: V,
        below_a: V,
        translation_b: V,
        below_b: V,
        c: V,
    }
    struct Builder;
    impl EscapesBuilder for Builder {
        type Escapes<V: Vector> = Escape<V>;
        fn new<V: Vector>() -> Self::Escapes<V> {
            Self::Escapes {
                translation_a: V::splat(88i8 as u8),
                below_a: V::splat(121i8 as u8),
                translation_b: V::splat(65i8 as u8),
                below_b: V::splat(124i8 as u8),
                c: V::splat(47i8 as u8),
            }
        }
    }
    impl<V: Vector> Escapes for Escape<V> {
        const ESCAPE_LEN: usize = 6usize;
        const FALSE_POSITIVE: bool = true;
        type Vector = V;
        fn masking(&self, vector2: V) -> V {
            vector2
                .add(self.translation_a)
                .gt(self.below_a)
                .or(vector2.add(self.translation_b).gt(self.below_b))
                .or(vector2.cmpeq(self.c))
        }
        fn escape(i: usize) -> &'static str {
            V_ESCAPE_QUOTES[i]
        }
        fn position(i: u8) -> usize {
            V_ESCAPE_CHARS[i as usize] as usize
        }
        fn byte_byte_compare(c: u8) -> bool {
            (V_ESCAPE_CHARS[c as usize] as usize) < V_ESCAPE_LEN
        }
    }
    escape_builder!(Builder);

    #[test]
    fn test_false_positive() {
        let mut buffer = String::new();
        let haystack = ">".to_string() + &"foobar".repeat(100) + "<";
        escape_string(&haystack, &mut buffer);
        assert_eq!(buffer, "&gt;".to_string() + &"foobar".repeat(100) + "&lt;");
    }
}
