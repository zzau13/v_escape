use v_latexescape::LatexEscape;

#[test]
fn test_escape() {
    let empty = "";
    let escapes = "#$%&\\^_{}~";
    let escaped = "\\#\\$\\%\\&\\textbackslash{}\\textasciicircum{}\\_\\{\\}\\textasciitilde{}";
    let string_long: &str = &"foobar".repeat(1024);

    assert_eq!(LatexEscape::from(empty).to_string(), empty);

    assert_eq!(LatexEscape::from("").to_string(), "");
    assert_eq!(LatexEscape::from("#$%&").to_string(), "\\#\\$\\%\\&");
    assert_eq!(
        LatexEscape::from("bar_^").to_string(),
        "bar\\_\\textasciicircum{}"
    );
    assert_eq!(LatexEscape::from("{foo}").to_string(), "\\{foo\\}");
    assert_eq!(
        LatexEscape::from("~\\").to_string(),
        "\\textasciitilde{}\\textbackslash{}"
    );
    assert_eq!(
        LatexEscape::from("_% of do$llar an&d #HASHES {I} have in ~ \\ latex").to_string(),
        "\\_\\% of do\\$llar an\\&d \\#HASHES \\{I\\} have in \\textasciitilde{} \
         \\textbackslash{} latex"
    );
    assert_eq!(
        LatexEscape::from(
            "_% of do$llar an&d #HASHES {I} have in ~ \\ latex"
                .repeat(10_000)
                .as_ref()
        )
        .to_string(),
        "\\_\\% of do\\$llar an\\&d \\#HASHES \\{I\\} have in \\textasciitilde{} \
         \\textbackslash{} latex"
            .repeat(10_000)
    );
    assert_eq!(
        LatexEscape::from("#".repeat(16).as_ref()).to_string(),
        "\\#".repeat(16)
    );
    assert_eq!(
        LatexEscape::from("#".repeat(32).as_ref()).to_string(),
        "\\#".repeat(32)
    );
    assert_eq!(
        LatexEscape::from("#".repeat(64).as_ref()).to_string(),
        "\\#".repeat(64)
    );
    assert_eq!(
        LatexEscape::from("#".repeat(128).as_ref()).to_string(),
        "\\#".repeat(128)
    );
    assert_eq!(
        LatexEscape::from("#".repeat(1024).as_ref()).to_string(),
        "\\#".repeat(1024)
    );
    assert_eq!(
        LatexEscape::from("#".repeat(129).as_ref()).to_string(),
        "\\#".repeat(129)
    );
    assert_eq!(
        LatexEscape::from("#".repeat(128 * 2 - 1).as_ref()).to_string(),
        "\\#".repeat(128 * 2 - 1)
    );
    assert_eq!(
        LatexEscape::from("#".repeat(128 * 8 - 1).as_ref()).to_string(),
        "\\#".repeat(128 * 8 - 1)
    );
    assert_eq!(
        LatexEscape::from(string_long.as_ref()).to_string(),
        string_long
    );
    assert_eq!(
        LatexEscape::from([string_long, "#"].join("").as_ref()).to_string(),
        [string_long, "\\#"].join("")
    );
    assert_eq!(
        LatexEscape::from(["#", string_long].join("").as_ref()).to_string(),
        ["\\#", string_long].join("")
    );
    assert_eq!(
        LatexEscape::from(escapes.repeat(1024).as_ref()).to_string(),
        escaped.repeat(1024)
    );
    assert_eq!(
        LatexEscape::from(
            [string_long, &escapes.repeat(13)]
                .join("")
                .repeat(1024)
                .as_ref()
        )
        .to_string(),
        [string_long, &escaped.repeat(13)].join("").repeat(1024)
    );
    assert_eq!(
        LatexEscape::from([string_long, "#", string_long].join("").as_ref()).to_string(),
        [string_long, "\\#", string_long].join("")
    );
    assert_eq!(
        LatexEscape::from(
            [string_long, "#", string_long, escapes, string_long,]
                .join("")
                .as_ref()
        )
        .to_string(),
        [string_long, "\\#", string_long, escaped, string_long,].join("")
    );

    let string_short = "Lorem ipsum dolor sit amet,#foo>bar&foo\"bar\\foo/bar";
    let string_short_escaped =
        "Lorem ipsum dolor sit amet,\\#foo>bar\\&foo\"bar\\textbackslash{}foo/bar";
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

    assert_eq!(LatexEscape::from(no_escape.as_ref()).to_string(), no_escape);
    assert_eq!(
        LatexEscape::from(no_escape_long.as_ref()).to_string(),
        no_escape_long
    );
    assert_eq!(
        LatexEscape::from(string_short.as_ref()).to_string(),
        string_short_escaped
    );
    assert_eq!(
        LatexEscape::from(string_short.repeat(1024).as_ref()).to_string(),
        string_short_escaped.repeat(1024)
    );
    assert_eq!(
        LatexEscape::from(string_short.as_ref()).to_string(),
        string_short_escaped
    );

    // Size
    assert_eq!(
        LatexEscape::from("#".repeat(16).as_ref()).size(),
        "\\#".repeat(16).len()
    );
    assert_eq!(
        LatexEscape::from("#".repeat(32).as_ref()).size(),
        "\\#".repeat(32).len()
    );
    assert_eq!(
        LatexEscape::from("#".repeat(64).as_ref()).size(),
        "\\#".repeat(64).len()
    );
    assert_eq!(
        LatexEscape::from("#".repeat(128).as_ref()).size(),
        "\\#".repeat(128).len()
    );
    assert_eq!(
        LatexEscape::from("#".repeat(1024).as_ref()).size(),
        "\\#".repeat(1024).len()
    );
    assert_eq!(
        LatexEscape::from("#".repeat(129).as_ref()).size(),
        "\\#".repeat(129).len()
    );
    assert_eq!(
        LatexEscape::from("#".repeat(128 * 2 - 1).as_ref()).size(),
        "\\#".repeat(128 * 2 - 1).len()
    );
    assert_eq!(
        LatexEscape::from("#".repeat(128 * 8 - 1).as_ref()).size(),
        "\\#".repeat(128 * 8 - 1).len()
    );
    assert_eq!(
        LatexEscape::from(string_short.as_ref()).size(),
        string_short_escaped.len()
    );
    assert_eq!(
        LatexEscape::from([string_short, "#"].join("").as_ref()).size(),
        [string_short_escaped, "\\#"].join("").len()
    );
    assert_eq!(
        LatexEscape::from(["#", string_short].join("").as_ref()).size(),
        ["\\#", string_short_escaped].join("").len()
    );
    assert_eq!(
        LatexEscape::from(escapes.repeat(1024).as_ref()).size(),
        escaped.repeat(1024).len()
    );
    assert_eq!(
        LatexEscape::from(
            [string_short, &escapes.repeat(13)]
                .join("")
                .repeat(1024)
                .as_ref()
        )
        .size(),
        [string_short_escaped, &escaped.repeat(13)]
            .join("")
            .repeat(1024)
            .len()
    );
}
