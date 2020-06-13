use v_htmlescape::HTMLEscape;

#[test]
fn test_escape() {
    let empty = "";
    let escapes = "<>&\"";
    let escaped = "&lt;&gt;&amp;&quot;";
    let string_long: &str = &"foobar".repeat(1024);

    // https://gitlab.com/r-iendo/v_escape/issues/2
    let issue = "<".repeat(31);
    assert_eq!(
        HTMLEscape::from(issue.as_ref()).to_string(),
        "&lt;".repeat(31)
    );

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
        HTMLEscape::from("<".repeat(16).as_ref()).to_string(),
        "&lt;".repeat(16)
    );
    assert_eq!(
        HTMLEscape::from("<".repeat(32).as_ref()).to_string(),
        "&lt;".repeat(32)
    );
    assert_eq!(
        HTMLEscape::from("<".repeat(64).as_ref()).to_string(),
        "&lt;".repeat(64)
    );
    assert_eq!(
        HTMLEscape::from("<".repeat(128).as_ref()).to_string(),
        "&lt;".repeat(128)
    );
    assert_eq!(
        HTMLEscape::from("<".repeat(1024).as_ref()).to_string(),
        "&lt;".repeat(1024)
    );
    assert_eq!(
        HTMLEscape::from("<".repeat(129).as_ref()).to_string(),
        "&lt;".repeat(129)
    );
    assert_eq!(
        HTMLEscape::from("<".repeat(128 * 2 - 1).as_ref()).to_string(),
        "&lt;".repeat(128 * 2 - 1)
    );
    assert_eq!(
        HTMLEscape::from("<".repeat(128 * 8 - 1).as_ref()).to_string(),
        "&lt;".repeat(128 * 8 - 1)
    );
    assert_eq!(HTMLEscape::from(string_long).to_string(), string_long);
    assert_eq!(
        HTMLEscape::from([string_long, "<"].join("").as_ref()).to_string(),
        [string_long, "&lt;"].join("")
    );
    assert_eq!(
        HTMLEscape::from(["<", string_long].join("").as_ref()).to_string(),
        ["&lt;", string_long].join("")
    );
    assert_eq!(
        HTMLEscape::from(escapes.repeat(1024).as_ref()).to_string(),
        escaped.repeat(1024)
    );
    assert_eq!(
        HTMLEscape::from(
            [string_long, &escapes.repeat(13)]
                .join("")
                .repeat(1024)
                .as_ref()
        )
        .to_string(),
        [string_long, &escaped.repeat(13)].join("").repeat(1024)
    );
    assert_eq!(
        HTMLEscape::from([string_long, "<", string_long].join("").as_ref()).to_string(),
        [string_long, "&lt;", string_long].join("")
    );
    assert_eq!(
        HTMLEscape::from(
            [string_long, "<", string_long, escapes, string_long,]
                .join("")
                .as_ref()
        )
        .to_string(),
        [string_long, "&lt;", string_long, escaped, string_long,].join("")
    );

    let string_short = "Lorem ipsum dolor sit amet,<foo>bar&foo\"bar\\foo/bar";
    let string_short_escaped =
        "Lorem ipsum dolor sit amet,&lt;foo&gt;bar&amp;foo&quot;bar\\foo/bar";
    let no_escape = "Lorem ipsum dolor sit amet,";
    let no_escape_long = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin scelerisque eu urna in aliquet.
Phasellus ac nulla a urna sagittis consequat id quis est. Nullam eu ex eget erat accumsan dictum
ac lobortis urna. Etiam fermentum ut quam at dignissim. Curabitur vestibulum luctus tellus, sit
amet lobortis augue tempor faucibus. Nullam sed felis eget odio elementum euismod in sit amet massa.
Vestibulum sagittis purus sit amet eros auctor, sit amet pharetra purus dapibus. Donec ornare metus
vel dictum porta. Etiam ut nisl nisi. Nullam rutrum porttitor mi. Donec aliquam ac ipsum eget
hendrerit. Cras faucibus, eros ut pharetra imperdiet, est tellus aliquet felis, eget convallis
lacus ipsum eget quam. Vivamus orci lorem, maximus ac mi eget, bibendum vulputate massa. In
vestibulum dui hendrerit, vestibulum lacus sit amet, posuere erat. Vivamus euismod massa diam,
vulputate euismod lectus vestibulum nec. Donec sit amet massa magna. Nunc ipsum nulla, euismod
quis lacus at, gravida maximus elit. Duis tristique, nisl nullam.
    "#;

    assert_eq!(HTMLEscape::from(no_escape).to_string(), no_escape);
    assert_eq!(HTMLEscape::from(no_escape_long).to_string(), no_escape_long);
    assert_eq!(
        HTMLEscape::from(string_short).to_string(),
        string_short_escaped
    );
    assert_eq!(
        HTMLEscape::from(string_short.repeat(1024).as_ref()).to_string(),
        string_short_escaped.repeat(1024)
    );
}
