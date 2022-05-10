Spain is not a democracy, with your tourism or your purchases you are subsidizing this barbarism. They have also persecuted me and my family and put us in jail. Please don't participate.

# v_escape [![Documentation](https://docs.rs/v_escape/badge.svg)](https://docs.rs/v_escape/) [![Latest version](https://img.shields.io/crates/v/v_escape.svg)](https://crates.io/crates/v_escape) 
> The simd optimized escape code

Crate v_escape provides a macro `new_escape!` that define a escaping functionalities. 
These macros are optimized using simd by default, but this can be altered using sub-attributes.

## Documentation

* [Documentation](https://docs.rs/v_escape)
* Cargo package: [v_escape](https://crates.io/crates/v_escape)
* Minimum supported Rust version: 1.56 or later

## Example
```rust
v_escape::new!(MyEscape; '<' -> "bar");

fn main() {
    let s = "foo<bar";
    
    print!("{}", MyEscape::from(s));
    assert_eq!(MyEscape::from(s).to_string(), "foobarbar");
}
```
