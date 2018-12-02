# v_htmlescape [![Documentation](https://docs.rs/v_htmlescape/badge.svg)](https://docs.rs/v_htmlescape/) [![Latest version](https://img.shields.io/crates/v/v_htmlescape.svg)](https://crates.io/crates/v_htmlescape) [![codecov](https://codecov.io/gh/rust-iendo/v_htmlescape/branch/master/graph/badge.svg)](https://codecov.io/gh/rust-iendo/v_htmlescape) [![Build status](https://api.travis-ci.org/rust-iendo/v_htmlescape.svg?branch=master)](https://travis-ci.org/rust-iendo/v_htmlescape) [![Windows build](https://ci.appveyor.com/api/projects/status/github/rust-iendo/v_htmlescape?svg=true)](https://ci.appveyor.com/project/botika/v-htmlescape)
> The simd optimized html escape code
# Quick start
 
```rust
extern crate v_htmlescape;
use v_htmlescape::Escape;

print!("{}", Escape::new("foo<bar".as_bytes()));
```
