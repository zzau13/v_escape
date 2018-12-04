//! # Quick start
//!
//! ```
//! extern crate v_htmlescape;
//!
//! print!("{}", v_htmlescape::Escape::new("foo<bar".as_bytes()));
//! ```
//!
#[macro_use]
extern crate cfg_if;

use std::str;

#[macro_use]
mod utils;

use utils::*;

#[macro_export]
macro_rules! new_escape {
    ($name:ident, $fun:ident) => {
        use std::fmt::{self, Display, Formatter};

        /// Html escape formatter
        pub struct $name<'a> {
            bytes: &'a [u8],
        }

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
                    $fun(self.bytes, fmt)
                }
            }
        }
    };
}

new_escape!(Escape, _imp);

cfg_if! {
    if #[cfg(all(target_arch = "x86_64", not(target_os = "windows"), v_htmlescape_simd))] {

        use std::mem;
        use std::sync::atomic::{AtomicUsize, Ordering};
        pub mod avx;
        pub mod sse;

        #[inline(always)]
        fn _imp(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
            // https://github.com/BurntSushi/rust-memchr/blob/master/src/x86/mod.rs#L9-L29
            static mut FN: fn(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result = detect;

            fn detect(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
                let fun = if cfg!(v_htmlescape_avx) && is_x86_feature_detected!("avx2") {
                    avx::escape as usize
                } else if cfg!(v_htmlescape_sse) && is_x86_feature_detected!("sse4.2") {
                    sse::escape as usize
                } else {
                    escape as usize
                };

                let slot = unsafe { &*(&FN as *const _ as *const AtomicUsize) };
                slot.store(fun as usize, Ordering::Relaxed);
                unsafe {
                    mem::transmute::<usize, fn(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result>(fun)(bytes, fmt)
                }
            }

            unsafe {
                let slot = &*(&FN as *const _ as * const AtomicUsize);
                let fun = slot.load(Ordering::Relaxed);
                mem::transmute::<usize, fn(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result>(fun)(bytes, fmt)
            }
        }
    } else {

        #[inline(always)]
        fn _imp(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
            escape(bytes, fmt)
        }
    }
}

/// Scalar html escape
#[inline]
pub fn escape(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
    let mut start = 0;

    for (i, b) in bytes.iter().enumerate() {
        bodies!(i, *b, start, fmt, bytes, escape_body);
    }

    fmt.write_str(unsafe { str::from_utf8_unchecked(&bytes[start..]) })?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    new_escape!(SEscape, escape);

    #[test]
    fn test_escape() {
        let empty = "";
        assert_eq!(SEscape::new(empty.as_bytes()).to_string(), empty);

        assert_eq!(SEscape::new("".as_bytes()).to_string(), "");
        assert_eq!(SEscape::new("<&>".as_bytes()).to_string(), "&lt;&amp;&gt;");
        assert_eq!(SEscape::new("bar&".as_bytes()).to_string(), "bar&amp;");
        assert_eq!(SEscape::new("<foo".as_bytes()).to_string(), "&lt;foo");
        assert_eq!(SEscape::new("bar&h".as_bytes()).to_string(), "bar&amp;h");
        assert_eq!(
            SEscape::new("// my <html> is \"unsafe\" & should be 'escaped'".as_bytes()).to_string(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be &#x27;escaped&#x27;"
        );
    }
}
