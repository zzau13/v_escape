# v_htmlescape [![Documentation](https://docs.rs/v_htmlescape/badge.svg)](https://docs.rs/v_htmlescape/) [![Latest version](https://img.shields.io/crates/v/v_htmlescape.svg)](https://crates.io/crates/v_htmlescape)
> The simd optimized html escape code
# Quick start
 
```rust
extern crate v_htmlescape;
use v_htmlescape::HTMLEscape;

print!("{}", HTMLEscape::new(b"foo<bar"));
```
