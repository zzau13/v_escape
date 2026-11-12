#[cfg(any(feature = "string", feature = "fmt", feature = "bytes"))]
const INPUT: &str = "&<>\"'/7?";
#[cfg(any(feature = "string", feature = "fmt", feature = "bytes"))]
const EXPECTED: &str = "&amp;&lt;&gt;&quot;&#x27;/7?";

#[cfg(feature = "string")]
#[test]
fn escapes_only_html_sensitive_characters_in_strings() {
    let mut output = String::new();
    v_html5escape::escape_string(INPUT, &mut output);
    assert_eq!(output, EXPECTED);
}

#[cfg(feature = "fmt")]
#[test]
fn escapes_only_html_sensitive_characters_in_formatting() {
    assert_eq!(v_html5escape::escape_fmt(INPUT).to_string(), EXPECTED);
}

#[cfg(feature = "bytes")]
#[test]
fn escapes_only_html_sensitive_characters_in_bytes() {
    let input = [INPUT.as_bytes(), &[0xff]].concat();
    let expected = [EXPECTED.as_bytes(), &[0xff]].concat();
    let mut output = Vec::new();
    v_html5escape::escape_bytes(&input, &mut output);
    assert_eq!(output, expected);
}
