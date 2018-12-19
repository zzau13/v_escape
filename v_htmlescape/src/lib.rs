//! # Quick start
//!
//! ```rust
//! extern crate v_htmlescape;
//! use v_htmlescape::HTMLEscape;
//!
//! print!("{}", HTMLEscape::new(b"foo<bar"));
//! ```
//!

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate v_escape;

cfg_if! {
    if #[cfg(all(v_htmlescape_simd, v_htmlescape_avx))] {
        new_escape_sized!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f; || ",
            avx = true, simd = true
        );
    } else if #[cfg(all(v_htmlescape_simd, v_htmlescape_sse))] {
        new_escape_sized!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f; || ",
            avx = false, simd = true
        );
    } else {
        new_escape_sized!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f; || ",
            avx = false, simd = false
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_escape() {
        let empty = "";
        assert_eq!(HTMLEscape::new(empty.as_bytes()).to_string(), empty);

        assert_eq!(HTMLEscape::new("".as_bytes()).to_string(), "");
        assert_eq!(
            HTMLEscape::new("<&>".as_bytes()).to_string(),
            "&lt;&amp;&gt;"
        );
        assert_eq!(HTMLEscape::new("bar&".as_bytes()).to_string(), "bar&amp;");
        assert_eq!(HTMLEscape::new("<foo".as_bytes()).to_string(), "&lt;foo");
        assert_eq!(HTMLEscape::new("bar&h".as_bytes()).to_string(), "bar&amp;h");
        assert_eq!(
            HTMLEscape::new("// my <html> is \"unsafe\" & should be 'escaped'".as_bytes())
                .to_string(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be &#x27;escaped&#x27;"
        );
        assert_eq!(
            HTMLEscape::new(
                "// my <html> is \"unsafe\" & should be 'escaped'"
                    .repeat(10_000)
                    .as_bytes()
            )
            .to_string(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be &#x27;escaped&#x27;"
                .repeat(10_000)
        );
    }

    #[test]
    fn test_size() {
        let empty = "";
        assert_eq!(HTMLEscape::new(empty.as_bytes()).size(), empty.len());

        assert_eq!(HTMLEscape::new("".as_bytes()).size(), 0);
        assert_eq!(
            HTMLEscape::new("<&>".as_bytes()).size(),
            "&lt;&amp;&gt;".len()
        );
        assert_eq!(HTMLEscape::new("bar&".as_bytes()).size(), "bar&amp;".len());
        assert_eq!(HTMLEscape::new("<foo".as_bytes()).size(), "&lt;foo".len());
        assert_eq!(
            HTMLEscape::new("bar&h".as_bytes()).size(),
            "bar&amp;h".len()
        );
        assert_eq!(
            HTMLEscape::new("// my <html> is \"unsafe\" & should be 'escaped'".repeat(10_000).as_bytes()).size(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; should be &#x27;escaped&#x27;".repeat(10_000).len()
        );
    }
}
