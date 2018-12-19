//! # Quick start
//!
//! ```
//! #[macro_use]
//! extern crate v_escape;
//!
//! new_escape_sized!(MyEscape, "62->bar || ");
//!
//! # fn main() {
//! # let s = b"foo>bar";
//! let escaped = MyEscape::new(s);
//!
//! print!("#{} : {}", escaped.size(), escaped);
//! # }
//! ```
//!
//! > build.rs
//! ```rust
//! use version_check::is_min_version;
//!
//! fn main() {
//!     enable_simd_optimizations();
//! }
//!
//! fn enable_simd_optimizations() {
//!     if !is_min_version("1.27.0").map_or(false, |(yes, _)| yes) {
//!         println!("cargo:rustc-cfg=v_escape_nosimd");
//!     }
//! }
//! ```
//!
#![allow(unused_imports)]

use v_escape_derive::Escape;

#[macro_use]
mod loops;
#[macro_use]
mod macros;
#[macro_use]
mod scalar;
#[macro_use]
mod sse;
#[macro_use]
mod avx;

#[macro_export]
macro_rules! _v_escape_escape_new {
    ($name:ident) => {
        #[allow(dead_code)]
        impl<'a> $name<'a> {
            pub fn new(s: &[u8]) -> $name {
                $name { bytes: s }
            }
        }

        impl<'a> Display for $name<'a> {
            fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
                #[allow(unused_unsafe)]
                unsafe {
                    _escape(self.bytes, fmt)
                }
            }
        }
    };
}

#[macro_export]
/// Generate code for new escape struct
macro_rules! new_escape {
    ($name:ident, $pairs:expr) => {
        use std::fmt::{self, Display, Formatter};
        new_escape!(imp $name, $pairs);
        _v_escape_escape_new!($name);
    };
    ($name:ident, $pairs:expr, $($t:tt)+) => {
        use std::fmt::{self, Display, Formatter};
        new_escape!(imp $name, $pairs, $($t)+);
        _v_escape_escape_new!($name);
    };
    (imp $name:ident, $pairs:expr) => {
        #[derive(Escape)]
        #[escape(pairs = $pairs)]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }
    };
    (imp $name:ident, $pairs:expr, $($t:tt)+) => {
        #[derive(Escape)]
        #[escape(pairs = $pairs, $($t)+)]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }
    };
}

#[macro_export]
/// Generate code for new escape struct with size method
macro_rules! new_escape_sized {
    ($name:ident, $pairs:expr) => {
        use std::fmt::{self, Display, Formatter};

        new_escape_sized!(imp $name, $pairs);
        _v_escape_escape_new!($name);

        impl<'a> $name<'a> {
            pub fn size(&self) -> usize {
                #[allow(unused_unsafe)]
                unsafe {
                    _size(self.bytes)
                }
            }
        }
    };
    ($name:ident, $pairs:expr, $($t:tt)+) => {
        use std::fmt::{self, Display, Formatter};

        new_escape_sized!(imp $name, $pairs, $($t)+);
        _v_escape_escape_new!($name);

        impl<'a> $name<'a> {
            pub fn size(&self) -> usize {
                #[allow(unused_unsafe)]
                unsafe {
                    _size(self.bytes)
                }
            }
        }
    };
    (imp $name:ident, $pairs:expr) => {
        #[derive(Escape)]
        #[escape(pairs = $pairs, sized = "true")]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }
    };
    (imp $name:ident, $pairs:expr, $($t:tt)+) => {
        #[derive(Escape)]
        #[escape(pairs = $pairs, sized = "true", $($t)+)]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }
    };
}

#[macro_export]
macro_rules! _v_escape_cfg_escape {
    ($simd:expr, $avx:expr) => {
        #[cfg(all(
            target_arch = "x86_64",
            not(all(target_os = "windows", v_escape_nosimd))
        ))]
        #[inline(always)]
        // https://github.com/BurntSushi/rust-memchr/blob/master/src/x86/mod.rs#L9-L29
        fn _escape(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
            use std::mem;
            use std::sync::atomic::{AtomicUsize, Ordering};
            static mut FN: fn(&[u8], &mut Formatter) -> fmt::Result = detect;

            fn detect(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
                let fun = if $simd && $avx && is_x86_feature_detected!("avx2") {
                    avx::escape as usize
                } else if $simd && is_x86_feature_detected!("sse4.2") {
                    sse::escape as usize
                } else {
                    scalar::escape as usize
                };

                let slot = unsafe { &*(&FN as *const _ as *const AtomicUsize) };
                slot.store(fun as usize, Ordering::Relaxed);
                unsafe {
                    mem::transmute::<usize, fn(&[u8], &mut Formatter) -> fmt::Result>(fun)(
                        bytes, fmt,
                    )
                }
            }

            unsafe {
                let slot = &*(&FN as *const _ as *const AtomicUsize);
                let fun = slot.load(Ordering::Relaxed);
                mem::transmute::<usize, fn(&[u8], &mut Formatter) -> fmt::Result>(fun)(bytes, fmt)
            }
        }

        #[cfg(not(all(
            target_arch = "x86_64",
            not(all(target_os = "windows", v_escape_nosimd))
        )))]
        #[inline(always)]
        fn _escape(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
            scalar::escape(bytes, fmt)
        }
    };
}

#[macro_export]
macro_rules! _v_escape_cfg_sized {
    ($simd:expr, $avx:expr) => {
        #[cfg(all(
            target_arch = "x86_64",
            not(all(target_os = "windows", v_escape_nosimd))
        ))]
        #[inline(always)]
        // https://github.com/BurntSushi/rust-memchr/blob/master/src/x86/mod.rs#L9-L29
        fn _size(bytes: &[u8]) -> usize {
            use std::mem;
            use std::sync::atomic::{AtomicUsize, Ordering};
            static mut FN: fn(&[u8]) -> usize = detect;

            fn detect(bytes: &[u8]) -> usize {
                let fun = if $simd && $avx && is_x86_feature_detected!("avx2") {
                    avx::size as usize
                } else if $simd && is_x86_feature_detected!("sse4.2") {
                    sse::size as usize
                } else {
                    scalar::size as usize
                };

                let slot = unsafe { &*(&FN as *const _ as *const AtomicUsize) };
                slot.store(fun as usize, Ordering::Relaxed);
                unsafe { mem::transmute::<usize, fn(&[u8]) -> usize>(fun)(bytes) }
            }

            unsafe {
                let slot = &*(&FN as *const _ as *const AtomicUsize);
                let fun = slot.load(Ordering::Relaxed);
                mem::transmute::<usize, fn(&[u8]) -> usize>(fun)(bytes)
            }
        }

        #[cfg(not(all(
            target_arch = "x86_64",
            not(all(target_os = "windows", v_escape_nosimd))
        )))]
        #[inline(always)]
        fn _size(bytes: &[u8]) -> usize {
            scalar::size(bytes)
        }
    };
}
