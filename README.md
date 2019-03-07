# v_escape [![Documentation](https://docs.rs/v_escape/badge.svg)](https://docs.rs/v_escape/) [![Latest version](https://img.shields.io/crates/v/v_escape.svg)](https://crates.io/crates/v_escape) 
> The simd optimized escape code

Crate v_escape provides a macro `new_escape!` that define a `struct` with 
escaping functionalities. These macros are optimized using simd by default, 
but this can be alter using sub-attributes.

# Quick start
In order to use v_escape you will have to call one of the two macros
to create a escape `struct`. In this example, when using the macro
`new_escape!(MyEscape, "62->bar");` a new a `struct` `MyEscape`
will be created that every time its method `MyEscape::fmt` is called
will replace all characters `">"` with `"bar"`.
 
```rust
#[macro_use]
extern crate v_escape;

new_escape!(MyEscape, "62->bar");

fn main() {
    let s = "foo<bar";
    let escaped = MyEscape::from(s);
    
    print!("{}", escaped);
}
```

To check if rust version has simd functionality. The following code
has to be added to file `build.rs`.
```rust
use v_escape::check_version;

fn main() {
    check_version();
}
```

## Support

[Patreon](https://www.patreon.com/r_iendo)