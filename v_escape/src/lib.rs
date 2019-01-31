//! Crate v_escape provides a macro, `new_escape!` that define a `struct` with
//! escaping functionality. These macros are optimized using simd by default,
//! but this can be alter using sub-attributes.
//!
//! # Quick start
//! In order to use v_escape you will have to call one of the two macros
//! to create a escape `struct`. In this example, when using the macro
//! `new_escape!(MyEscape, "62->bar");` a new a `struct` `MyEscape`
//! will be created that every time its method `MyEscape::fmt` is called
//! will replace all characters `">"` with `"bar"`.
//!
//! ```
//! #[macro_use]
//! extern crate v_escape;
//!
//! new_escape!(MyEscape, "62->bar");
//!
//! # fn main() {
//! # let s = "foo>bar";
//! let escaped = escape(s);
//!
//! print!("{}", escaped);
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
//! and several can be defined, referred as `Pairs`. The syntax to define
//! `Pairs` consists of a character, followed
//! by the delimiter `->`, followed by the substitution quote
//! and the delimiter ` || ` (last delimiter is optional):
//!
//!    `([character]->[quote] || )*`
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
//! new_escape!(MyEscape, "49->bar");
//! # fn main() {
//! assert_eq!(escape("foo 1").to_string(), "foo bar");
//! # }
//! ```
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape!(MyEscape, "0x31->bar");
//! # fn main() {
//! assert_eq!(escape("foo 1").to_string(), "foo bar");
//! # }
//! ```
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape!(MyEscape, "0o61->bar");
//! # fn main() {
//! assert_eq!(escape("foo 1").to_string(), "foo bar");
//! # }
//! ```
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape!(MyEscape, "#1->bar");
//! # fn main() {
//! assert_eq!(escape("foo 1").to_string(), "foo bar");
//! # }
//! ```
//!
//! `ranges` refers to a possible optimization that when given a number of pairs,
//! an optimal number of ranges (with a maximum of three) are calculated and used
//! to escape all bytes within each range. It is possible the existence of false
//! positives, and will be discarded. If a lot of pairs have to be discarded
//! sub-attribute `ranges` must be explicitly disabled. In the case that more than
//! three bytes are used and the distance between the escaped characters is very
//! large, then `ranges = false`.
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape!(MyEscape, "0-> || 33->foo || 66->bar || 127->", ranges = false);
//! # fn main() {
//! assert_eq!(escape("fooBbar").to_string(), "foobarbar");
//! # }
//! ```
//!
//! In the following example more than 16 pairs are given, this exceeds simd's
//! boundary. If simd optimization is wanted, ranges must be enabled (default)
//! or an error will be thrown. It is possible to not use ranges but simd
//! optimization has to be disabled.
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape!(
//!     MyEscape,
//!     "62->b || 60->f || B->b || 65->f || 0o67->b || #6->f || 68->b || \
//!     71->f || 72->b || 73->f || 74->b || 75->f || 76->b || 77->f || \
//!     78->b || 79->f || 0x1A->f"
//! );
//! # fn main() {
//! assert_eq!(escape("foo>bar<").to_string(), "foobbarf");
//! # }
//! ```
//!
//! For debugging purposes, sub-attribute `print`, can be set to `true`
//! to print generated code
//!
//! ```
//! # #[macro_use]
//! # extern crate v_escape;
//! new_escape!(MyEscape, "o->bar", print = true);
//! # fn main() {
//! # assert_eq!(escape("foo").to_string(), "fbarbar");
//! # }
//! ```
//!
#![allow(unused_imports)]

use v_escape_derive::Escape;

#[macro_use]
mod macros;
#[macro_use]
mod scalar;
#[macro_use]
mod sse;
#[macro_use]
mod ranges;

#[macro_export]
/// Generates struct `$name` with escaping functionality at `fmt`
///
/// It will get as input:
///
/// * $__name__: Name of escape class.
///
/// * $__pairs__: Pairs of `[character]->[quote] || [character]->[quote]` or
///              `[character]->[quote]`.
///
/// * $__t__: Optional boolean parameters (simd, avx, sse, print).
///     * __simd__:  If true (by default), simd optimizations are enabled. When false,
///         no matter value of avx, `sse4.2` will be used,
///     * __avx__:   If true (by default), avx optimization are enabled. When false,
///         `sse2`(if `ranges=true` and `simd=true`) or `scalar`(if `simd=false`) will be used.
///     * __ranges__:   If true (by default), ranges optimizations are enabled. When false,
///         `sse4.2`(if `simd=true`) or `scalar`(if `simd=false`) will be used.
///     * __print__: If true (false by default), prints out generated code to console.
///
/// and will:
///
/// 1. Import `std::fmt::{self, Display, Formatter}`
///
/// 2. Define basic struct with attribute `bytes` and `Escape`
///    derive functionality
///
/// 3. Implements for `$name` constructors `new` and `From<&'a str>`
///
/// 4. Implements trait `Display` for `$name` with escape functionality
///
/// 5. Implements function `escape(&str) -> $name`
///
/// #### Example
///
/// ```
/// #[macro_use]
/// extern crate v_escape;
///
/// new_escape!(MyEscape, "o->bar");
///
/// # fn main() {
/// assert_eq!(escape("foobar").to_string(), "fbarbarbar");
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
#[doc(hidden)]
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

        #[inline]
        pub fn escape(s: &str) -> $name {
            $name::from(s)
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
#[doc(hidden)]
/// cfg_if for escape function
macro_rules! _v_escape_cfg_escape {
    (false, $a:expr, $b:expr) => {
        _v_escape_cfg_escape!(fn);
    };
    (true, false, false) => {
        _v_escape_cfg_escape!(fn);
    };
    (true, $($t:tt)+) => {
        #[cfg(all(target_arch = "x86_64", not(v_escape_nosimd)))]
        #[inline(always)]
        // https://github.com/BurntSushi/rust-memchr/blob/master/src/x86/mod.rs#L9-L29
        fn _escape(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
            use std::mem;
            use std::sync::atomic::{AtomicUsize, Ordering};
            static mut FN: fn(&[u8], &mut Formatter) -> fmt::Result = detect;

            fn detect(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
                let fun = _v_escape_cfg_escape!(if $($t)+);

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

        #[cfg(not(all(target_arch = "x86_64", not(v_escape_nosimd))))]
        _v_escape_cfg_escape!(fn);
    };
    (fn) => {
        #[inline(always)]
        fn _escape(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
            scalar::escape(bytes, fmt)
        }
    };
    (if false, $a:expr) => {
        if is_x86_feature_detected!("sse4.2") {
            sse::escape as usize
        } else {
            scalar::escape as usize
        }
    };
    (if true, true) => {
        if is_x86_feature_detected!("avx2") {
            ranges::avx::escape as usize
        } else if is_x86_feature_detected!("sse2") {
            ranges::sse::escape as usize
        } else {
            scalar::escape as usize
        }
    };
    (if true, false) => {
        if is_x86_feature_detected!("sse2") {
            ranges::sse::escape as usize
        } else {
            scalar::escape as usize
        }
    };
}
