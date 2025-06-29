#![cfg(all(feature = "fmt", feature = "string"))]

use v_escape::escape;

escape! {
    '"' -> "&quot;",
    '<' -> "&lt;"
}

#[test]
fn test() {
    let s = "Hello,< world!\"";
    let escaped = escape_fmt(s).to_string();
    assert_eq!(escaped, "Hello,&lt; world!&quot;");
    let mut escaped = String::with_capacity(s.len());
    escape_string(&s, &mut escaped);
    assert_eq!(escaped, "Hello,&lt; world!&quot;");
}
