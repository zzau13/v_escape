# [![Documentation](https://docs.rs/v_escape/badge.svg)](https://docs.rs/v_escape/) [![Latest version](https://img.shields.io/crates/v/v_escape.svg)](https://crates.io/crates/v_escape)

# v_escape

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

## Documentation

- Minimum supported Rust version: 1.85.0 or later

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
