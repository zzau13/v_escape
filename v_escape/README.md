# V_escape

This crate provides a procedural macro for generating escape functions.

## Example

```rust
# #![cfg(all(feature = "fmt", feature = "string"))]
use v_escape::escape;

escape! {
    b'"' -> "&quot;",
    b'<' -> "&lt;"
}

let s = "Hello,< world!\"";
let escaped = escape_fmt(s).to_string();
assert_eq!(escaped, "Hello,&lt; world!&quot;");
let mut escaped = String::with_capacity(s.len());
escape_string(&s, &mut escaped);
assert_eq!(escaped, "Hello,&lt; world!&quot;");
```

## Features

- `fmt`: Enables the `escape_fmt` function.
- `string`: Enables the `escape_string` function.
- `bytes`: Enables the `escape_bytes` function.
- `std`: Enables the `std` library features.
- `alloc`: Enables the `alloc` library features.
