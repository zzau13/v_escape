# Disclaimer
Due to the high degree of idiocy on forums like reddit, I recommend that you use another solution. I can't ban it by license, but I hope this is enough.

# v_escape [![Documentation](https://docs.rs/v_escape/badge.svg)](https://docs.rs/v_escape/) [![Latest version](https://img.shields.io/crates/v/v_escape.svg)](https://crates.io/crates/v_escape) [![Build Status](https://travis-ci.org/botika/v_escape.svg?branch=master)](https://travis-ci.org/botika/v_escape)
> The simd optimized escape code

Crate v_escape provides a macro `new_escape!` that define a escaping functionalities. 
These macros are optimized using simd by default, but this can be altered using sub-attributes.

## Documentation

* [Documentation](https://docs.rs/v_escape)
* Cargo package: [v_escape](https://crates.io/crates/v_escape)
* Minimum supported Rust version: 1.42 or later

## Example
```rust
new_escape!(MyEscape, "62->bar");

fn main() {
    let s = "foo<bar";
    
    print!("{}", MyEscape::from(s));
}
```
