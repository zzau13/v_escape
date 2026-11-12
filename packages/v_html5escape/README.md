[![Documentation](https://docs.rs/v_html5escape/badge.svg)](https://docs.rs/v_html5escape/)
[![Latest version](https://img.shields.io/crates/v/v_html5escape.svg)](https://crates.io/crates/v_html5escape)

# v_html5escape

SIMD-accelerated escaping of the five HTML-sensitive characters: `&`, `<`,
`>`, `"`, and `'`. Unlike `v_htmlescape`, this crate leaves `/` unchanged.

This escaping is intended for HTML text and quoted attribute values. It is not
a substitute for context-specific escaping in JavaScript, CSS, or URLs.

## Quick start

```rust
use v_html5escape::escape_fmt;

print!("{}", escape_fmt("<p title=\"it's\">&</p>"));
```

The generated crate provides `escape_fmt`, `escape_string`, and `escape_bytes`,
gated by the `fmt`, `string`, and `bytes` features respectively.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
