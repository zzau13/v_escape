#[test]
fn tests() {
    use std::borrow::Cow;
    use std::char::from_u32;
    use v_latexescape::VLatexescape;
    use v_latexescape::{escape, ranges, scalar};
    fn all_utf8_less(less: &str) -> String {
        assert_eq!(less.len(), less.as_bytes().len());
        let less = less.as_bytes();
        let mut buf = String::with_capacity(204_672 - less.len());
        for i in 0..0x80u8 {
            if !less.contains(&i) {
                buf.push(from_u32(i as u32).unwrap())
            }
        }
        for i in 0x80..0xD800 {
            buf.push(from_u32(i).unwrap());
        }
        for i in 0xE000..0x11000 {
            buf.push(from_u32(i).unwrap());
        }
        buf
    }
    let empty = "";
    let escapes = "#$%&\\^_{}~";
    let escaped = "\\#\\$\\%\\&\\textbackslash{}\\textasciicircum{}\\_\\{\\}\\textasciitilde{}";
    let utf8: &str = &all_utf8_less("#$%&\\^_{}~");
    let empty_heap = String::new();
    let short = "foobar";
    let string_long: &str = &short.repeat(1024);
    let string = "#$%&\\^_{}~".to_string();
    let cow = Cow::Owned("#$%&\\^_{}~".to_string());
    let buf = scalar::escape(&[short, escapes, short].join("")).to_string();
    assert_eq!(buf, [short, escaped, short].join(""));
    struct FAvx(String);
    impl std::fmt::Display for FAvx {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unsafe { ranges::avx::escape(&self.0.as_bytes(), f) }
        }
    }
    if is_x86_feature_detected!("avx2") {
        let buf = FAvx([short, escapes, short].join("")).to_string();
        assert_eq!(buf, [short, escaped, short].join(""));
    }
    struct FSse(String);
    impl std::fmt::Display for FSse {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unsafe { ranges::sse::escape(&self.0.as_bytes(), f) }
        }
    }
    if is_x86_feature_detected!("sse2") {
        let buf = FSse([short, escapes, short].join("")).to_string();
        assert_eq!(buf, [short, escaped, short].join(""));
    }
    #[cfg(feature = "bytes-buf")]
    unsafe {
        use v_latexescape::b_escape;
        let mut buf = String::new();
        b_escape([short, escapes, short].join("").as_bytes(), &mut buf);
        assert_eq!(buf, [short, escaped, short].join(""));
        let mut buf = String::new();
        scalar::b_escape([short, escapes, short].join("").as_bytes(), &mut buf);
        assert_eq!(buf, [short, escaped, short].join(""));
        if is_x86_feature_detected!("avx2") {
            let mut buf = String::new();
            ranges::avx::b_escape([short, escapes, short].join("").as_bytes(), &mut buf);
            assert_eq!(buf, [short, escaped, short].join(""));
        }
        if is_x86_feature_detected!("sse2") {
            let mut buf = String::new();
            ranges::sse::b_escape([short, escapes, short].join("").as_bytes(), &mut buf);
            assert_eq!(buf, [short, escaped, short].join(""));
        }
    }
    assert_eq!(VLatexescape::from(empty).to_string(), empty);
    assert_eq!(VLatexescape::from(escapes).to_string(), escaped);
    assert_eq!(escape(&empty_heap).to_string(), empty);
    assert_eq!(escape(&cow).to_string(), escaped);
    assert_eq!(escape(&string).to_string(), escaped);
    assert_eq!(escape(&utf8).to_string(), utf8);
    assert_eq!(VLatexescape::from(string_long).to_string(), string_long);
    assert_eq!(
        VLatexescape::from(escapes.repeat(1024).as_ref()).to_string(),
        escaped.repeat(1024)
    );
    assert_eq!(
        VLatexescape::from([short, escapes, short].join("").as_ref()).to_string(),
        [short, escaped, short].join("")
    );
    assert_eq!(
        VLatexescape::from([escapes, short].join("").as_ref()).to_string(),
        [escaped, short].join("")
    );
    assert_eq!(
        VLatexescape::from(["f", escapes, short].join("").as_ref()).to_string(),
        ["f", escaped, short].join("")
    );
    assert_eq!(
        VLatexescape::from(["f", escapes].join("").as_ref()).to_string(),
        ["f", escaped].join("")
    );
    assert_eq!(
        VLatexescape::from(["fo", escapes].join("").as_ref()).to_string(),
        ["fo", escaped].join("")
    );
    assert_eq!(
        VLatexescape::from(["fo", escapes, "b"].join("").as_ref()).to_string(),
        ["fo", escaped, "b"].join("")
    );
    assert_eq!(
        VLatexescape::from(escapes.repeat(2).as_ref()).to_string(),
        escaped.repeat(2)
    );
    assert_eq!(
        VLatexescape::from(escapes.repeat(3).as_ref()).to_string(),
        escaped.repeat(3)
    );
    assert_eq!(
        VLatexescape::from(["f", &escapes.repeat(2)].join("").as_ref()).to_string(),
        ["f", &escaped.repeat(2)].join("")
    );
    assert_eq!(
        VLatexescape::from(["fo", &escapes.repeat(2)].join("").as_ref()).to_string(),
        ["fo", &escaped.repeat(2)].join("")
    );
    assert_eq!(
        VLatexescape::from(["fo", &escapes.repeat(2), "bar"].join("").as_ref()).to_string(),
        ["fo", &escaped.repeat(2), "bar"].join("")
    );
    assert_eq!(
        VLatexescape::from(["fo", &escapes.repeat(3), "bar"].join("").as_ref()).to_string(),
        ["fo", &escaped.repeat(3), "bar"].join("")
    );
    assert_eq!(
        VLatexescape::from([&escapes.repeat(3), "bar"].join("").as_ref()).to_string(),
        [&escaped.repeat(3), "bar"].join("")
    );
    assert_eq!(
        VLatexescape::from([short, &escapes.repeat(3), "bar"].join("").as_ref()).to_string(),
        [short, &escaped.repeat(3), "bar"].join("")
    );
    assert_eq!(
        VLatexescape::from([short, &escapes.repeat(5), "bar"].join("").as_ref()).to_string(),
        [short, &escaped.repeat(5), "bar"].join("")
    );
    assert_eq!(
        VLatexescape::from(
            [string_long, &escapes.repeat(13)]
                .join("")
                .repeat(1024)
                .as_ref()
        )
        .to_string(),
        [string_long, &escaped.repeat(13)].join("").repeat(1024)
    );
    assert_eq!(
        VLatexescape::from([utf8, escapes, short].join("").as_ref()).to_string(),
        [utf8, escaped, short].join("")
    );
    assert_eq!(
        VLatexescape::from([utf8, escapes, utf8].join("").as_ref()).to_string(),
        [utf8, escaped, utf8].join("")
    );
    assert_eq!(
        VLatexescape::from([&utf8.repeat(124), escapes, utf8].join("").as_ref()).to_string(),
        [&utf8.repeat(124), escaped, utf8].join("")
    );
    assert_eq!(
        VLatexescape::from(
            [escapes, &utf8.repeat(124), escapes, utf8]
                .join("")
                .as_ref()
        )
        .to_string(),
        [escaped, &utf8.repeat(124), escaped, utf8].join("")
    );
    assert_eq!(
        VLatexescape::from(
            [escapes, &utf8.repeat(124), escapes, utf8, escapes]
                .join("")
                .as_ref()
        )
        .to_string(),
        [escaped, &utf8.repeat(124), escaped, utf8, escaped].join("")
    );
}
