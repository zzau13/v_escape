//! Crate v_escape provides two macros, `new_escape!` and `new_escape_sized!`,
//! that define a `struct` with escaping functionalities. These macros are
//! optimized using simd by default, but this can be alter using sub-attributes.
//!
//! # Quick start
//! In order to use v_escape you will have to call one of the two macros
//! to create a escape `struct`. In this example, when using the macro
//! `new_escape_sized!(MyEscape, "62->bar");` a new a `struct` `MyEscape`
//! will be created that every time its method `MyEscape::from` is called
//! will replace all characters `">"` with `"bar"`.
//! ```
//! #[macro_use]
//! extern crate v_escape;
//!
//! new_escape_sized!(MyEscape, "62->bar");
//!
//! # fn main() {
//! # let s = "foo>bar";
//! let escaped = MyEscape::from(s);
//!
//! print!("#{} : {}", escaped.size(), escaped);
//! # }
//! ```
//!
//! To check if rust version has simd functionality. The following code
//! has to be added to file `build.rs`.
//!
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
//! v_escape uses a simple syntax to replace characters
//! with their respective quotes. The tuple is named `Pair`,
//! and several can be defined, refered as `Pairs`. The syntax to define
//! `Pairs` consists of a character, followed
//! by the delimiter `->`, followed by the substitution quote
//! and the delimiter ` || ` (last delimiter is optional):
//!
//!    `( [character]->[quote] || )*`
//!
//! * `character` :   Character to substitute. Accepts`i8+` from `0` to `i8::MAX` and
//!                 accepts the following formats: decimal (49), hexadecimal (0x31),
//!                 octal (0o61) or character (#1).
//!                 Note: Numbers are read in ASCII: `#6->foo`
//!
//! * `quote` :   Characters that will replace `character`
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(MyEscape, "49->bar");
//! # fn main() {
//! assert_eq!(MyEscape::from("foo 1").to_string(), "foo bar");
//! # }
//! ```
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(MyEscape, "0x31->bar");
//! # fn main() {
//! assert_eq!(MyEscape::from("foo 1").to_string(), "foo bar");
//! # }
//! ```
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(MyEscape, "0o61->bar");
//! # fn main() {
//! assert_eq!(MyEscape::from("foo 1").to_string(), "foo bar");
//! # }
//! ```
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(MyEscape, "#1->bar");
//! # fn main() {
//! assert_eq!(MyEscape::from("foo 1").to_string(), "foo bar");
//! # }
//! ```
//!
//! The sub-attribute `simd`, true by default, can process a maximum
//! of 16 `Pairs`. If more `Pairs` are needed, simd optimizations has
//! to be deactivated using sub-attribute `simd = false`
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(
//!     MyEscape,
//!     "62->b || 60->f || A->b || 65->f || 0o67->b || #6->f || 68->b || \
//!     71->f || 72->b || 73->f || 74->b || 75->f || 76->b || 77->f || \
//!     78->b || 79->f || 0x1A->f",
//!     simd = false
//! );
//! # fn main() {
//! assert_eq!(MyEscape::from("foo>bar<").to_string(), "foobbarf");
//! # }
//! ```
//!
//! Optimization for avx requires the creation of ranges. If the
//! distance between the escaped characters is very large,
//! sub-attribute `avx = false` should be disabled
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape!(MyEscape, "0-> || 33->foo || 66->bar || 127->", avx = false);
//! # fn main() {
//! assert_eq!(MyEscape::from("fooBbar").to_string(), "foobarbar");
//! # }
//! ```
//!
//! For debugging purposes, sub-attribute `print`, can be set to `true`
//! to print generated code
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape_sized!(MyEscape, "o->bar", print = true);
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
/// Macro new_escape generates struct `$name` with escaping functionality.
///
/// It will get as input:
///
/// * $__name__: Name of escape class.
///
/// * $__pairs__: Pairs of `[character]->[quote] || [character]->[quote]` or
///              `[character]->[quote]`.
///
/// * $__t__: Optional boolean parameters (simd, avx, print).
///     * __simd__:  If true (by default), simd optimizations are enabled. When false,
///          no matter value of avx, `sse4.2` will be used,
///     * __avx__:   If true (by default), avx optimization are enabled. When false,
///          `sse4.2`(if `simd=true`) or `scalar`(if `simd=false`) will be used.
///     * __print__: If true (false by default), prints out generated code to console.
///
/// and will:
///
/// 1. Import fmt, Display and Formatter.
///
/// 2. Define basic struct with attribute `bytes` and escaping
///    derive functionality.
///
/// 3. Implements a constructor `new` and traits `Display` and
///    `From<&'a str>` to struct `$name` (using `_v_escape_escape_new` macro).
///
/// The generated struct `$name` can escape a string using function `from`
/// and can be displayed as in the example.
///
/// #### Example
///
/// ```
/// #[macro_use]
/// extern crate v_escape;
///
/// new_escape_sized!(MyEscape, "o->bar", print=true);
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
    // Macro called without attributes
    ($name:ident, $pairs:expr) => {
        use std::fmt::{self, Display, Formatter};

        // Adding code generated by Derive  with Escape and escape
        #[derive(Escape)]
        #[escape(pairs = $pairs)]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }

        // Implementing function new and traits Display and From
        _v_escape_escape_new!($name);
    };
    // Macro called with attributes
    ($name:ident, $pairs:expr, $($t:tt)+) => {
        use std::fmt::{self, Display, Formatter};

        // Adding code generated by Derive  with Escape and escape
        // passing attributes to function escape
        #[derive(Escape)]
        #[escape(pairs = $pairs, $($t)+)]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }

        // Implementing function new and traits Display and From
        _v_escape_escape_new!($name);
    };
}

#[macro_export]
/// Escape implementation
///
/// Generates function new, and traits From and Display, for class `$name`
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
/// Generate code for new escape struct with size method
///
/// Not use with empty quote `"i8->"`
///
/// Follows same process as new_escape, but with a function
/// called size implemented at the end of the defined struct.
///
/// It will get as input:
///
/// * $__name__: Name of escape class.
///
/// * $__pairs__: Pairs of `[character]->[quote] || [character]->[quote]` or
///              `[character]->[quote]`.
///
/// * $__t__: Optional boolean parameters (simd, avx, print).
///     * __simd__:  If true (by default), `simd` optimizations are enabled
///                 (if `avx=true`) or `sse4.2` (if `avx=false`). When false,
///                 `scalar` will be used.
///     * __avx__:  Disabling `avx` optimizations (`true` by default).
///     * __print__: If `true` (`false` by default), prints out generated code to console.
///
/// |  | `avx=true` | `avx=false` |
/// |--:|----------|-----------|
/// |  __`simd=true`__ | `simd` (default) | `sse4.2` |
/// | __`simd=false`__ | `scalar` | `scalar` |
///
/// and will:
///
/// 1. Import `fmt`, `Display` and `Formatter`.
///
/// 2. Define basic struct with attribute `bytes` and escaping
///    derive functionality.
///
/// 3. Implements a constructor `new` and traits `Display` and
///    `From<&'a str>` to struct `$name` (using `_v_escape_escape_new` macro).
///
/// The generated struct `$name` can escape a string using function
/// `from` and the escaped string can be displayed as well as the
/// size of it as in the example:
///
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
    // Macro called without attributes
    ($name:ident, $pairs:expr) => {
        use std::fmt::{self, Display, Formatter};

        // Adding code generated by Derive  with Escape and escape
        #[derive(Escape)]
        #[escape(pairs = $pairs, sized = true)]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }

        // Implementing function new and traits Display and From
        _v_escape_escape_new!($name);

        // Implementing function size
        impl<'a> $name<'a> {
            pub fn size(&self) -> usize {
                #[allow(unused_unsafe)]
                unsafe {
                    _size(self.bytes)
                }
            }
        }
    };
    // Macro called with attributes
    ($name:ident, $pairs:expr, $($t:tt)+) => {
        use std::fmt::{self, Display, Formatter};

        // Adding code generated by Derive  with Escape and escape
        // passing attributes to function escape
        #[derive(Escape)]
        #[escape(pairs = $pairs, sized = true, $($t)+)]
        pub struct $name<'a> {
            bytes: &'a [u8],
        }

        // Implementing function new and traits Display and From
        _v_escape_escape_new!($name);

        // Implementing function size
        impl<'a> $name<'a> {
            pub fn size(&self) -> usize {
                #[allow(unused_unsafe)]
                unsafe {
                    _size(self.bytes)
                }
            }
        }
    };
}

#[macro_export]
/// cfg_if for escape function
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
/// cfg_if for size function
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
