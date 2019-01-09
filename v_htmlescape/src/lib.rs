//! # Quick start
//!
//! ```rust
//! extern crate v_htmlescape;
//! use v_htmlescape::HTMLEscape;
//!
//! print!("{}", HTMLEscape::from("foo<bar"));
//! ```
//!

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate v_escape;

cfg_if! {
    if #[cfg(all(v_htmlescape_simd, v_htmlescape_avx))] {
        new_escape!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f;",
            avx = true, simd = true
        );

        pub mod sized {
            new_escape_sized!(
                HTMLEscape,
                "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f;",
                avx = true, simd = true
            );
        }
    } else if #[cfg(all(v_htmlescape_simd, v_htmlescape_sse))] {
        new_escape!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f;",
            avx = false, simd = true
        );

        pub mod sized {
            new_escape_sized!(
                HTMLEscape,
                "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f;",
                avx = false, simd = true
            );
        }
    } else {
        new_escape!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f;",
            avx = false, simd = false
        );

        pub mod sized {
            new_escape_sized!(
                HTMLEscape,
                "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f;",
                avx = false, simd = false
            );
        }
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
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be &#x27;escaped&#x27;"
        );
        assert_eq!(
            HTMLEscape::from(
                "// my <html> is \"unsafe\" & should be 'escaped'"
                    .repeat(10_000)
                    .as_ref()
            )
            .to_string(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be &#x27;escaped&#x27;"
                .repeat(10_000)
        );
    }

    #[test]
    fn test_size() {
        use super::sized::HTMLEscape;

        let empty = "";
        assert_eq!(HTMLEscape::from(empty).size(), empty.len());

        assert_eq!(HTMLEscape::from("").size(), 0);
        assert_eq!(HTMLEscape::from("<&>").size(), "&lt;&amp;&gt;".len());
        assert_eq!(HTMLEscape::from("bar&").size(), "bar&amp;".len());
        assert_eq!(HTMLEscape::from("<foo").size(), "&lt;foo".len());
        assert_eq!(HTMLEscape::from("bar&h").size(), "bar&amp;h".len());
        assert_eq!(
            HTMLEscape::from(
                "// my <html> is \"unsafe\" & should be 'escaped'"
                    .repeat(10_000)
                    .as_ref()
            )
            .size(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be &#x27;escaped&#x27;"
                .repeat(10_000)
                .len()
        );
    }
}
