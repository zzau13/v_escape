# v_htmlescape [![Documentation](https://docs.rs/v_htmlescape/badge.svg)](https://docs.rs/v_htmlescape/) [![Latest version](https://img.shields.io/crates/v/v_htmlescape.svg)](https://crates.io/crates/v_htmlescape) [![codecov](https://codecov.io/gh/rust-iendo/v_htmlescape/branch/master/graph/badge.svg)](https://codecov.io/gh/rust-iendo/v_htmlescape) [![Build status](https://api.travis-ci.org/rust-iendo/v_htmlescape.svg?branch=master)](https://travis-ci.org/rust-iendo/v_htmlescape) [![Windows build](https://ci.appveyor.com/api/projects/status/github/rust-iendo/v_htmlescape?svg=true)](https://ci.appveyor.com/project/botika/v-htmlescape)
> The simd optimized html escape code
# Quick start
 
```rust
extern crate v_htmlescape;
use v_htmlescape::HTMLEscape;

print!("{}", HTMLEscape::from("foo<bar"));
```

# v_escape [![Documentation](https://docs.rs/v_escape/badge.svg)](https://docs.rs/v_escape/) [![Latest version](https://img.shields.io/crates/v/v_escape.svg)](https://crates.io/crates/v_escape)
> The simd optimized escape code
# Quick start
 
```rust
#[macro_use]
extern crate v_escape;

new_escape_sized!(MyEscape, "62->bar");

fn main() {
    let s = "foo<bar";
    let escaped = MyEscape::from(s);
    
    print!("#{} : {}", escaped.size(), escaped);
}
```

> build.rs
```rust
use version_check::is_min_version;

fn main() {
    enable_simd_optimizations();
}

fn enable_simd_optimizations() {
    if !is_min_version("1.27.0").map_or(false, |(yes, _)| yes) {
        println!("cargo:rustc-cfg=v_escape_nosimd");
    }
}
```
