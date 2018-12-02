extern crate v_htmlescape;

use v_htmlescape::Escape;

#[test]
fn test_escape() {
    let escapes = "<>&\"'/";
    let escaped = "&lt;&gt;&amp;&quot;&#x27;&#x2f;";
    let string_long: &str = &"foobar".repeat(1024);

    assert_eq!(Escape::new("".as_bytes()).to_string(), "");
    assert_eq!(Escape::new("<&>".as_bytes()).to_string(), "&lt;&amp;&gt;");
    assert_eq!(Escape::new("bar&".as_bytes()).to_string(), "bar&amp;");
    assert_eq!(Escape::new("<foo".as_bytes()).to_string(), "&lt;foo");
    assert_eq!(Escape::new("bar&h".as_bytes()).to_string(), "bar&amp;h");
    assert_eq!(
        Escape::new("// my <html> is \"unsafe\" & should be 'escaped'".as_bytes()).to_string(),
        "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
         should be &#x27;escaped&#x27;"
    );
    assert_eq!(
        Escape::new(&"<".repeat(16).as_bytes()).to_string(),
        "&lt;".repeat(16)
    );
    assert_eq!(
        Escape::new(&"<".repeat(32).as_bytes()).to_string(),
        "&lt;".repeat(32)
    );
    assert_eq!(
        Escape::new(&"<".repeat(64).as_bytes()).to_string(),
        "&lt;".repeat(64)
    );
    assert_eq!(
        Escape::new(&"<".repeat(128).as_bytes()).to_string(),
        "&lt;".repeat(128)
    );
    assert_eq!(
        Escape::new(&"<".repeat(1024).as_bytes()).to_string(),
        "&lt;".repeat(1024)
    );
    assert_eq!(
        Escape::new(&"<".repeat(129).as_bytes()).to_string(),
        "&lt;".repeat(129)
    );
    assert_eq!(
        Escape::new(&"<".repeat(128 * 2 - 1).as_bytes()).to_string(),
        "&lt;".repeat(128 * 2 - 1)
    );
    assert_eq!(
        Escape::new(&"<".repeat(128 * 8 - 1).as_bytes()).to_string(),
        "&lt;".repeat(128 * 8 - 1)
    );
    assert_eq!(Escape::new(string_long.as_bytes()).to_string(), string_long);
    assert_eq!(
        Escape::new(&[string_long, "<"].join("").as_bytes()).to_string(),
        [string_long, "&lt;"].join("")
    );
    assert_eq!(
        Escape::new(&["<", string_long].join("").as_bytes()).to_string(),
        ["&lt;", string_long].join("")
    );
    assert_eq!(
        Escape::new(&escapes.repeat(1024).as_bytes()).to_string(),
        escaped.repeat(1024)
    );
    assert_eq!(
        Escape::new(
            &[string_long, &escapes.repeat(13)]
                .join("")
                .repeat(1024)
                .as_bytes()
        ).to_string(),
        [string_long, &escaped.repeat(13)].join("").repeat(1024)
    );
    assert_eq!(
        Escape::new(&[string_long, "<", string_long].join("").as_bytes()).to_string(),
        [string_long, "&lt;", string_long].join("")
    );
    assert_eq!(
        Escape::new(
            &[string_long, "<", string_long, escapes, string_long,]
                .join("")
                .as_bytes()
        ).to_string(),
        [string_long, "&lt;", string_long, escaped, string_long,].join("")
    );
}
