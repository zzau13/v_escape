extern crate v_htmlescape;

use v_htmlescape::Escape;

#[test]
fn test_escape() {
    let empty = "";
    let escapes = "<>&\"'/";
    let escaped = "&lt;&gt;&amp;&quot;&#x27;&#x2f;";
    let string_long: &str = &"foobar".repeat(1024);

    assert_eq!(Escape::new(empty.as_bytes()).to_string(), empty);
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

    let string_long = r#"
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris consequat tellus sit
    amet ornare fermentum. Etiam nec erat ante. In at metus a orci mollis scelerisque.
    Sed eget ultrices turpis, at sollicitudin erat. Integer hendrerit nec magna quis
    venenatis. Vivamus non dolor hendrerit, vulputate velit sed, varius nunc. Quisque
    in pharetra mi. Sed ullamcorper nibh malesuada commodo porttitor. Ut scelerisque
    sodales felis quis dignissim. Morbi aliquam finibus justo, sit amet consectetur
    mauris efficitur sit amet. Donec posuere turpis felis, eu lacinia magna accumsan
    quis. Fusce egestas lacus vel fermentum tincidunt. Phasellus a nulla eget lectus
    placerat commodo at eget nisl. Fusce cursus dui quis purus accumsan auctor.
    Donec iaculis felis quis metus consectetur porttitor.
<p>
    Etiam nibh mi, <b>accumsan</b> quis purus sed, posuere fermentum lorem. In pulvinar porta
    maximus. Fusce tincidunt lacinia tellus sit amet tincidunt. Aliquam lacus est, pulvinar
    non metus a, <b>facilisis</b> ultrices quam. Nulla feugiat leo in cursus eleifend. Suspendisse
    eget nisi ac justo sagittis interdum id a ipsum. Nulla mauris justo, scelerisque ac
    rutrum vitae, consequat vel ex.
</p></p></p></p></p></p></p></p></p></p></p></p></p></p></p></p></p></p></p></p></p></p></p></p>
<p>
    Sed sollicitudin <b>sem</b> mauris, at rutrum nibh egestas vel. Ut eu nisi tellus. Praesent dignissim
    orci elementum, mattis turpis eget, maximus ante. Suspendisse luctus eu felis a tempor. Morbi
    ac risus vitae sem molestie ullamcorper. Curabitur ligula augue, sollicitudin quis maximus vel,
    facilisis sed nibh. Aenean auctor magna sem, id rutrum metus convallis quis. Nullam non arcu
    dictum, lobortis erat quis, rhoncus est. Suspendisse venenatis, mi sed venenatis vehicula,
    tortor dolor egestas lectus, et efficitur turpis odio non augue. Integer velit sapien, dictum
    non egestas vitae, hendrerit sed quam. Phasellus a nunc eu erat varius imperdiet. Etiam id
    sollicitudin turpis, vitae molestie orci. Quisque ornare magna quis metus rhoncus commodo.
    Phasellus non mauris velit.
</p>
<p>
    Etiam dictum tellus ipsum, nec varius quam ornare vel. Cras vehicula diam nec sollicitudin
    ultricies. Pellentesque rhoncus sagittis nisl id facilisis. Nunc viverra convallis risus ut
    luctus. Aliquam vestibulum <b>efficitur massa</b>, id tempus nisi posuere a. Aliquam scelerisque
    elit justo. Nullam a ante felis. Cras vitae lorem eu nisi feugiat hendrerit. Maecenas vitae
    suscipit leo, lacinia dignissim lacus. Sed eget volutpat mi. In eu bibendum neque. Pellentesque
    finibus velit a fermentum rhoncus. Maecenas leo purus, eleifend eu lacus a, condimentum sagittis
    justo.
</p>"#;
    let string_long_escaped = "\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris consequat tellus sit\n    amet ornare fermentum. Etiam nec erat ante. In at metus a orci mollis scelerisque.\n    Sed eget ultrices turpis, at sollicitudin erat. Integer hendrerit nec magna quis\n    venenatis. Vivamus non dolor hendrerit, vulputate velit sed, varius nunc. Quisque\n    in pharetra mi. Sed ullamcorper nibh malesuada commodo porttitor. Ut scelerisque\n    sodales felis quis dignissim. Morbi aliquam finibus justo, sit amet consectetur\n    mauris efficitur sit amet. Donec posuere turpis felis, eu lacinia magna accumsan\n    quis. Fusce egestas lacus vel fermentum tincidunt. Phasellus a nulla eget lectus\n    placerat commodo at eget nisl. Fusce cursus dui quis purus accumsan auctor.\n    Donec iaculis felis quis metus consectetur porttitor.\n&lt;p&gt;\n    Etiam nibh mi, &lt;b&gt;accumsan&lt;&#x2f;b&gt; quis purus sed, posuere fermentum lorem. In pulvinar porta\n    maximus. Fusce tincidunt lacinia tellus sit amet tincidunt. Aliquam lacus est, pulvinar\n    non metus a, &lt;b&gt;facilisis&lt;&#x2f;b&gt; ultrices quam. Nulla feugiat leo in cursus eleifend. Suspendisse\n    eget nisi ac justo sagittis interdum id a ipsum. Nulla mauris justo, scelerisque ac\n    rutrum vitae, consequat vel ex.\n&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;&lt;&#x2f;p&gt;\n&lt;p&gt;\n    Sed sollicitudin &lt;b&gt;sem&lt;&#x2f;b&gt; mauris, at rutrum nibh egestas vel. Ut eu nisi tellus. Praesent dignissim\n    orci elementum, mattis turpis eget, maximus ante. Suspendisse luctus eu felis a tempor. Morbi\n    ac risus vitae sem molestie ullamcorper. Curabitur ligula augue, sollicitudin quis maximus vel,\n    facilisis sed nibh. Aenean auctor magna sem, id rutrum metus convallis quis. Nullam non arcu\n    dictum, lobortis erat quis, rhoncus est. Suspendisse venenatis, mi sed venenatis vehicula,\n    tortor dolor egestas lectus, et efficitur turpis odio non augue. Integer velit sapien, dictum\n    non egestas vitae, hendrerit sed quam. Phasellus a nunc eu erat varius imperdiet. Etiam id\n    sollicitudin turpis, vitae molestie orci. Quisque ornare magna quis metus rhoncus commodo.\n    Phasellus non mauris velit.\n&lt;&#x2f;p&gt;\n&lt;p&gt;\n    Etiam dictum tellus ipsum, nec varius quam ornare vel. Cras vehicula diam nec sollicitudin\n    ultricies. Pellentesque rhoncus sagittis nisl id facilisis. Nunc viverra convallis risus ut\n    luctus. Aliquam vestibulum &lt;b&gt;efficitur massa&lt;&#x2f;b&gt;, id tempus nisi posuere a. Aliquam scelerisque\n    elit justo. Nullam a ante felis. Cras vitae lorem eu nisi feugiat hendrerit. Maecenas vitae\n    suscipit leo, lacinia dignissim lacus. Sed eget volutpat mi. In eu bibendum neque. Pellentesque\n    finibus velit a fermentum rhoncus. Maecenas leo purus, eleifend eu lacus a, condimentum sagittis\n    justo.\n&lt;&#x2f;p&gt;";
    let string_short = "Lorem ipsum dolor sit amet,<foo>bar&foo\"bar\\foo/bar";
    let string_short_escaped =
        "Lorem ipsum dolor sit amet,&lt;foo&gt;bar&amp;foo&quot;bar\\foo&#x2f;bar";
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

    assert_eq!(Escape::new(no_escape.as_bytes()).to_string(), no_escape);
    assert_eq!(
        Escape::new(no_escape_long.as_bytes()).to_string(),
        no_escape_long
    );
    assert_eq!(
        Escape::new(string_short.as_bytes()).to_string(),
        string_short_escaped
    );
    assert_eq!(
        Escape::new(string_short.repeat(1024).as_bytes()).to_string(),
        string_short_escaped.repeat(1024)
    );
    assert_eq!(
        Escape::new(string_long.as_bytes()).to_string(),
        string_long_escaped
    );

    // Size
    assert_eq!(
        Escape::new(&"<".repeat(16).as_bytes()).size(),
        "&lt;".repeat(16).len()
    );
    assert_eq!(
        Escape::new(&"<".repeat(32).as_bytes()).size(),
        "&lt;".repeat(32).len()
    );
    assert_eq!(
        Escape::new(&"<".repeat(64).as_bytes()).size(),
        "&lt;".repeat(64).len()
    );
    assert_eq!(
        Escape::new(&"<".repeat(128).as_bytes()).size(),
        "&lt;".repeat(128).len()
    );
    assert_eq!(
        Escape::new(&"<".repeat(1024).as_bytes()).size(),
        "&lt;".repeat(1024).len()
    );
    assert_eq!(
        Escape::new(&"<".repeat(129).as_bytes()).size(),
        "&lt;".repeat(129).len()
    );
    assert_eq!(
        Escape::new(&"<".repeat(128 * 2 - 1).as_bytes()).size(),
        "&lt;".repeat(128 * 2 - 1).len()
    );
    assert_eq!(
        Escape::new(&"<".repeat(128 * 8 - 1).as_bytes()).size(),
        "&lt;".repeat(128 * 8 - 1).len()
    );
    assert_eq!(
        Escape::new(string_long.as_bytes()).size(),
        string_long_escaped.len()
    );
    assert_eq!(
        Escape::new(&[string_long, "<"].join("").as_bytes()).size(),
        [string_long_escaped, "&lt;"].join("").len()
    );
    assert_eq!(
        Escape::new(&["<", string_long].join("").as_bytes()).size(),
        ["&lt;", string_long_escaped].join("").len()
    );
    assert_eq!(
        Escape::new(&escapes.repeat(1024).as_bytes()).size(),
        escaped.repeat(1024).len()
    );
    assert_eq!(
        Escape::new(
            &[string_long, &escapes.repeat(13)]
                .join("")
                .repeat(1024)
                .as_bytes()
        ).size(),
        [string_long_escaped, &escaped.repeat(13)]
            .join("")
            .repeat(1024)
            .len()
    );
}
