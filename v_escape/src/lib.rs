//! v_escape consists of two macros: `new_escape!` and `new_escape_sized!`.
//! A `struct` is generated implementing trait `Display`
//!
//! # Quick start
//!
//! ```
//! #[macro_use]
//! extern crate v_escape;
//!
//! new_escape_sized!(MyEscape, "62->bar || ");
//!
//! # fn main() {
//! # let s = "foo>bar";
//! let escaped = MyEscape::from(s);
//!
//! print!("#{} : {}", escaped.size(), escaped);
//! # }
//! ```
//!
//! > build.rs
//! ```ignore
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
//! ## Pairs syntax
//! v_escape uses a simple syntax to enter the characters and quotes
//! to replace them, which I will name `Pairs`.
//! The `Pairs` consist of the character, `i8+` and `0`, to the left
//! of the delimiter `->` followed by the substitution quote and
//! finalized by the delimiter ` || `
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(MyEscape, ">->bar || ");
//!
//! # fn main() {
//! assert_eq!(MyEscape::from("foo>bar").to_string(), "foobarbar");
//! # }
//! ```
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(MyEscape, ">->bar || <->foo || ");
//!
//! # fn main() {
//! assert_eq!(MyEscape::from("foo>bar<").to_string(), "foobarbarfoo");
//! # }
//! ```
//!
//! The maximum number of `Pairs` with activated simd is 16.
//! If you want more you can deactivate the optimizations by simd with
//! sub-attribute `simd = false`
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(
//!     MyEscape,
//!     "62->b || 60->f || 63->b || 65->f || 67->b || 66->f || 68->b || \
//!     71->f || 72->b || 73->f || 74->b || 75->f || 76->b || 77->f || \
//!     78->b || 79->f || 1->f || ",
//!     simd = false
//! );
//!
//! # fn main() {
//! assert_eq!(MyEscape::from("foo>bar<").to_string(), "foobbarf");
//! # }
//! ```
//!
//! Optimizations for avx require the creation of ranges. So if the
//! distance between your characters is very large you should disable
//! avx with sub-attribute `avx = false`
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape!(MyEscape, "0-> || 33->foo || 66->bar || 127-> || ", avx = false);
//!
//! # fn main() {
//! assert_eq!(MyEscape::from("fooBbar").to_string(), "foobarbar");
//! # }
//! ```
//!
//! For debug reasons you can print the code generated with the
//! sub-attribute `print = true`
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(MyEscape, "o->bar || ", print = true);
//! # fn main() {
//! # assert_eq!(MyEscape::from("foo").to_string(), "fbarbar");
//! # }
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
            pub fn new(bytes: &[u8]) -> $name {
                $name { bytes }
            }
        }

        impl<'a> From<&'a str> for $name<'a> {
            fn from(s: &str) -> $name {
                $name {
                    bytes: s.as_bytes(),
                }
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
///
/// #### Example
///
/// ```
/// #[macro_use]
/// extern crate v_escape;
///
/// new_escape_sized!(MyEscape, "o->bar || ");
///
/// # fn main() {
/// # let s = "foobar";
/// let escaped = MyEscape::from(s);
///
/// print!("{}", escaped);
/// # }
/// ```
///
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
/// Not use with empty quote `"i8-> || "`
///
/// ```
/// #[macro_use]
/// extern crate v_escape;
///
/// new_escape_sized!(MyEscape, "o->bar || ");
///
/// # fn main() {
/// # let s = "foo>bar";
/// let escaped = MyEscape::from(s);
///
/// print!("#{} : {}", escaped.size(), escaped);
/// # }
/// ```
///
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
        #[escape(pairs = $pairs, sized = true)]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }
    };
    (imp $name:ident, $pairs:expr, $($t:tt)+) => {
        #[derive(Escape)]
        #[escape(pairs = $pairs, sized = true, $($t)+)]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }
    };
}

#[macro_export]
macro_rules! _v_escape_cfg_escape {
    (true, $avx:expr) => {
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
                let fun = if $avx && is_x86_feature_detected!("avx2") {
                    avx::escape as usize
                } else if is_x86_feature_detected!("sse4.2") {
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
        _v_escape_cfg_escape!(fn);
    };
    (false, $avx:expr) => {
        _v_escape_cfg_escape!(fn);
    };
    (fn) => {
        #[inline(always)]
        fn _escape(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
            scalar::escape(bytes, fmt)
        }
    };
}

#[macro_export]
macro_rules! _v_escape_cfg_sized {
    (true, $avx:expr) => {
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
                let fun = if $avx && is_x86_feature_detected!("avx2") {
                    avx::size as usize
                } else if is_x86_feature_detected!("sse4.2") {
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
        _v_escape_cfg_sized!(fn);
    };
    (false, $avx:expr) => {
        _v_escape_cfg_sized!(fn);
    };
    (fn) => {
        #[inline(always)]
        fn _size(bytes: &[u8]) -> usize {
            scalar::size(bytes)
        }
    };
}
