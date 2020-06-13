//! # Quick start
//!
//! ```
//! extern crate v_htmlescape;
//! use v_htmlescape::escape;
//!
//! print!("{}", escape("foo<bar"));
//! ```
//!
//! https://html.spec.whatwg.org/#serialising-html-fragments
//! Escaping a string (for the purposes of the algorithm above) consists of running the following steps:
//!
//! Replace any occurrence of the "&" character by the string "&amp;".
//!
//! Replace any occurrences of the U+00A0 NO-BREAK SPACE character by the string "&nbsp;".
//!
//! If the algorithm was invoked in the attribute mode, replace any occurrences of the """
//! character by the string "&quot;".
//!
//! If the algorithm was not invoked in the attribute mode, replace any occurrences of
//! the "<" character by the string "&lt;", and any occurrences of the ">" character
//! by the string "&gt;".
#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate v_escape;

/// Without simd optimizations
pub mod fallback {
    new_escape!(
        HTMLEscape,
        "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot;",
        simd = false
    );
}

cfg_if! {
    if #[cfg(all(v_htmlescape_simd, v_htmlescape_avx))] {
        new_escape!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot;",
            simd = true, avx = true
        );
    } else if #[cfg(all(v_htmlescape_simd, v_htmlescape_sse))] {
        new_escape!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot;",
            simd = true, avx = false
        );
    } else {
        pub use self::fallback::*;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_escape() {
        use super::HTMLEscape;

        let empty = "";
        assert_eq!(HTMLEscape::from(empty).to_string(), empty);

        assert_eq!(HTMLEscape::from("").to_string(), "");
        assert_eq!(HTMLEscape::from("<&>").to_string(), "&lt;&amp;&gt;");
        assert_eq!(HTMLEscape::from("bar&").to_string(), "bar&amp;");
        assert_eq!(HTMLEscape::from("<foo").to_string(), "&lt;foo");
        assert_eq!(HTMLEscape::from("bar&h").to_string(), "bar&amp;h");
        assert_eq!(
            HTMLEscape::from("// my <html> is \"unsafe\" & should be 'escaped'").to_string(),
            "// my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be 'escaped'"
        );
        assert_eq!(
            HTMLEscape::from(
                "// my <html> is \"unsafe\" & should be 'escaped'"
                    .repeat(10_000)
                    .as_ref()
            )
            .to_string(),
            "// my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be 'escaped'"
                .repeat(10_000)
        );
    }
}
