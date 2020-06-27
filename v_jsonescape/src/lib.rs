//! # Quick start
//!
//! ```
//! extern crate v_jsonescape;
//! use v_jsonescape::escape;
//!
//! print!("{}", escape("foo<bar"));
//! ```
//!
#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate v_escape;
// https://tools.ietf.org/id/draft-ietf-json-rfc4627bis-09.html#rfc.section.7
// https://github.com/serde-rs/json/blob/master/src/ser.rs#L2113-L2143
macro_rules! new_json {
    ($simd:expr, $avx:expr) => {
        new_escape!(
            JSONEscape,
            "0x00->\\u0000 || \
            0x01->\\u0001 || \
            0x02->\\u0002 || \
            0x03->\\u0003 || \
            0x04->\\u0004 || \
            0x05->\\u0005 || \
            0x06->\\u0006 || \
            0x07->\\u0007 || \
            0x08->\\b || \
            0x09->\\t || \
            0x0A->\\n || \
            0x0B->\\u000b || \
            0x0C->\\f || \
            0x0D->\\r || \
            0x0E->\\u000e || \
            0x0F->\\u000f || \
            0x10->\\u0010 || \
            0x11->\\u0011 || \
            0x12->\\u0012 || \
            0x13->\\u0013 || \
            0x14->\\u0014 || \
            0x15->\\u0015 || \
            0x16->\\u0016 || \
            0x17->\\u0017 || \
            0x18->\\u0018 || \
            0x19->\\u0019 || \
            0x1A->\\u001a || \
            0x1B->\\u001b || \
            0x1C->\\u001c || \
            0x1D->\\u001d || \
            0x1E->\\u001e || \
            0x1F->\\u001f || \
            0x22->\\\" || \
            0x5C->\\\\",
            simd = $simd,
            avx = $avx
        );
    };
}

/// Without simd optimizations
pub mod fallback {
    new_json!(false, false);
}

cfg_if! {
    if #[cfg(all(v_jsonescape_simd, v_jsonescape_avx))] {
        new_json!(true, true);
    } else if #[cfg(all(v_jsonescape_simd, v_jsonescape_sse))] {
        new_json!(true, false);
    } else {
        pub use self::fallback::*;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_escape() {
        use super::*;

        let empty = "";
        assert_eq!(escape(empty).to_string(), empty);
        let tests = &[
            ('n', "n"),
            ('"', "\\\""),
            ('\\', "\\\\"),
            ('/', "/"),
            ('\x08', "\\b"),
            ('\x0C', "\\f"),
            ('\n', "\\n"),
            ('\r', "\\r"),
            ('\t', "\\t"),
            ('\x0B', "\\u000b"),
            ('\u{3A3}', "\u{3A3}"),
        ];
        for (c, e) in tests {
            assert_eq!(escape_char(*c).to_string(), *e);
            assert_eq!(escape(&c.to_string()).to_string(), *e);
        }
        let tests = tests
            .into_iter()
            .fold((String::new(), String::new()), |mut acc, (c, e)| {
                acc.0.push(*c);
                acc.1.push_str(e);
                acc
            });

        assert_eq!(escape(&tests.0).to_string(), tests.1);
        assert_eq!(escape(&tests.0.repeat(2)).to_string(), tests.1.repeat(2));
        assert_eq!(escape(&tests.0.repeat(4)).to_string(), tests.1.repeat(4));
        assert_eq!(escape(&tests.0.repeat(16)).to_string(), tests.1.repeat(16));
        assert_eq!(
            escape(&tests.0.repeat(128)).to_string(),
            tests.1.repeat(128)
        );

        let tests = &[("", ""), ("foo", "foo")];
        for (c, e) in tests {
            assert_eq!(escape(c).to_string(), *e);
        }
    }
}
