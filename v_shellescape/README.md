# v_shellescape [![Documentation](https://docs.rs/v_shellescape/badge.svg)](https://docs.rs/v_shellescape/) [![Latest version](https://img.shields.io/crates/v/v_shellescape.svg)](https://crates.io/crates/v_shellescape)
> The simd optimized shell escape code
# Quick start
 
```rust
extern crate v_shellescape;
use v_shellescape::{unix, windows};

print!("{}", unix::ShellEscape::from("linker=gcc -L/foo -Wl,bar"));
print!("{}", windows::ShellEscape::from("linker=gcc -L/foo -Wl,bar"));
```
