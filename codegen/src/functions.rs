#[macro_use]
mod macros;
#[macro_use]
mod scalar;
#[macro_use]
mod ranges;
#[macro_use]
mod chars;

#[macro_export]
macro_rules! new {
    // Macro called without attributes
    ($name:ident; $($t:tt)+) => {
        $crate::derive!($($t)+);
        $crate::escape_new!($name);
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape implementation
///
/// Generates function new, and traits From and Display, for class `$name`
macro_rules! escape_new {
    ($name:ident) => {
        pub struct $name<'a> {
            bytes: &'a [u8],
        }

        impl<'a> $name<'a> {
            #[inline]
            pub fn new(bytes: &[u8]) -> $name {
                $name { bytes }
            }

            #[inline]
            pub fn f_escape(&self, buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
                #[allow(unused_unsafe)]
                unsafe {
                    _f_escape(self.bytes, buf)
                }
            }
        }

        impl<'a> From<&'a str> for $name<'a> {
            #[inline]
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

        impl<'a> std::fmt::Display for $name<'a> {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_unsafe)]
                unsafe {
                    _escape(self.bytes, fmt)
                }
            }
        }

        #[inline]
        pub fn escape_char(c: char) -> impl std::fmt::Display {
            struct EscapeChar(char);

            impl std::fmt::Display for EscapeChar {
                fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    chars::escape_char(self.0, fmt)
                }
            }

            EscapeChar(c)
        }

        #[inline]
        pub fn f_escape(s: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            #[allow(unused_unsafe)]
            unsafe {
                _f_escape(s, buf)
            }
        }

        #[inline]
        pub fn f_escape_char(c: char, buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            #[allow(unused_unsafe)]
            unsafe {
                chars::f_escape_char(c, buf)
            }
        }

        /// Escape byte slice to `Buffer`
        ///
        /// # SIGILL
        /// Can produce **SIGILL** if compile with `sse2` or `avx2` and execute without they
        /// Because not exist way to build multiple static allocations by type
        /// And it's very expensive check it in runtime
        /// https://github.com/rust-lang/rust/issues/57775
        #[inline]
        pub fn b_escape<B: $crate::Buffer>(s: &[u8], buf: &mut B) {
            #[allow(unused_unsafe)]
            unsafe {
                _b_escape(s, buf)
            }
        }

        /// Escape char to `buf-min::Buffer`
        #[inline]
        pub fn b_escape_char<B: $crate::Buffer>(s: char, buf: &mut B) {
            #[allow(unused_unsafe)]
            unsafe {
                chars::b_escape_char(s, buf)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// cfg_if for escape function
macro_rules! cfg_escape {
    (false, $($t:tt)+) => {
        $crate::cfg_escape!(fn);
    };
    (true, $($t:tt)+) => {
        #[cfg(target_arch = "x86_64")]
        #[inline(always)]
        // https://github.com/BurntSushi/rust-memchr/blob/master/src/x86/mod.rs#L9-L29
        fn _escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            use std::mem;
            use std::sync::atomic::{AtomicUsize, Ordering};
            use std::fmt::{self, Formatter};
            static mut FN: fn(&[u8], &mut Formatter) -> fmt::Result = detect;

            fn detect(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
                let fun = $crate::cfg_escape!(if $($t)+);

                let slot = unsafe { &*(&FN as *const _ as *const AtomicUsize) };
                slot.store(fun, Ordering::Relaxed);
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

        #[cfg(not(target_arch = "x86_64"))]
        $crate::cfg_escape!(fn);
    };
    (fn) => {
        #[inline(always)]
        fn _escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            scalar::escape(bytes, fmt)
        }
    };
    (if true) => {
        if is_x86_feature_detected!("avx2") {
            ranges::avx::escape as usize
        } else if is_x86_feature_detected!("sse2") {
            ranges::sse::escape as usize
        } else {
            scalar::escape as usize
        }
    };
    (if false) => {
        if is_x86_feature_detected!("sse2") {
            ranges::sse::escape as usize
        } else {
            scalar::escape as usize
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// cfg_if for escape function
macro_rules! cfg_escape_ptr {
    (false, $($t:tt)+) => {
        $crate::cfg_escape_ptr!(fn);
    };
    (true, $($t:tt)+) => {
        #[cfg(target_arch = "x86_64")]
        #[inline(always)]
        #[allow(unreachable_code)]
        // https://github.com/BurntSushi/rust-memchr/blob/master/src/x86/mod.rs#L9-L29
        pub unsafe fn _f_escape(bytes: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            use std::mem;
            use std::sync::atomic::{AtomicUsize, Ordering};
            static mut FN: fn(&[u8], &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> = detect;

            fn detect(bytes: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
                let fun = $crate::cfg_escape_ptr!(if $($t)+);

                let slot = unsafe { &*(&FN as *const _ as *const AtomicUsize) };
                slot.store(fun, Ordering::Relaxed);
                unsafe {
                    mem::transmute::<usize, fn(&[u8], &mut [std::mem::MaybeUninit<u8>]) -> Option<usize>>(fun)(
                        bytes, buf,
                    )
                }
            }

            unsafe {
                let slot = &*(&FN as *const _ as *const AtomicUsize);
                let fun = slot.load(Ordering::Relaxed);
                mem::transmute::<usize, fn(&[u8], &mut [std::mem::MaybeUninit<u8>]) -> Option<usize>>(fun)(bytes, buf)
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        $crate::cfg_escape_ptr!(fn);
    };
    (fn) => {
        #[inline(always)]
        pub unsafe fn _f_escape(bytes: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            scalar::f_escape(bytes, buf)
        }
    };
    (if true) => {
        if is_x86_feature_detected!("avx2") {
            ranges::avx::f_escape as usize
        } else if is_x86_feature_detected!("sse2") {
            ranges::sse::f_escape as usize
        } else {
            scalar::f_escape as usize
        }
    };
    (if false) => {
        if is_x86_feature_detected!("sse2") {
            ranges::sse::f_escape as usize
        } else {
            scalar::f_escape as usize
        }
    };
}

fn cfg_escape_bytes() {}

#[macro_export]
#[doc(hidden)]
/// cfg_if for escape function
macro_rules! cfg_escape_bytes {
    (false, $($t:tt)+) => {
        $crate::cfg_escape_bytes!(fn);
    };
    (true, $($t:tt)+) => {
        #[cfg(target_arch = "x86_64")]
        #[inline(always)]
        pub unsafe fn _b_escape<B: $crate::Buffer>(bytes: &[u8], buf: &mut B) {
            $crate::cfg_escape_bytes!(if $($t)+, bytes, buf)
        }

        #[cfg(not(all(target_arch = "x86_64", not(b_escape_nosimd))))]
        $crate::cfg_escape_bytes!(fn);
    };
    (fn) => {
        #[inline(always)]
        pub unsafe fn _b_escape<B: $crate::Buffer>(bytes: &[u8], buf: &mut B) {
            scalar::b_escape(bytes, buf)
        }
    };
    (if true, $bytes:ident, $buf:ident) => {{
        #[cfg(not(v_escape_avx))] {
            #[cfg(not(v_escape_sse))] {
                scalar::b_escape($bytes, $buf)
            }
            #[cfg(v_escape_sse)] {
                ranges::sse::b_escape($bytes, $buf)
            }
        }
        #[cfg(v_escape_avx)] {
            ranges::avx::b_escape($bytes, $buf)
        }
    }};
    (if false, $bytes:ident, $buf:ident) => {{
        #[cfg(not(v_escape_sse))] {
            scalar::b_escape($bytes, $buf)
        }
        #[cfg(v_escape_sse)] {
            ranges::sse::b_escape($bytes, $buf)
        }
    }};
}
