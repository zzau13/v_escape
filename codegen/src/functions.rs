use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn escape_new(name: &Ident) -> TokenStream {
    quote! {
        pub struct #name<'a> {
            bytes: &'a [u8],
        }

        impl<'a> #name<'a> {
            #[inline]
            pub fn new(bytes: &[u8]) -> #name {
                #name { bytes }
            }
        }

        impl<'a> From<&'a str> for #name<'a> {
            #[inline]
            fn from(s: &str) -> #name {
                #name {
                    bytes: s.as_bytes(),
                }
            }
        }

        #[inline]
        pub fn escape(s: &str) -> #name {
            #name::from(s)
        }

        impl<'a> std::fmt::Display for #name<'a> {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_unsafe)]
                unsafe {
                    _escape(self.bytes, fmt)
                }
            }
        }

        /// Escape byte slice to `Buffer`
        ///
        /// # SIGILL
        /// Can produce **SIGILL** if compile with `sse2` or `avx2` and execute without they
        /// Because not exist way to build multiple static allocations by type
        /// And it's very expensive check it in runtime
        /// https://github.com/rust-lang/rust/issues/57775
        #[cfg(feature = "bytes-buf")]
        #[inline]
        pub fn b_escape<B: buf_min::Buffer>(s: &[u8], buf: &mut B) {
            #[allow(unused_unsafe)]
            unsafe {
                _b_escape(s, buf)
            }
        }

        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        #[inline(always)]
        // https://github.com/BurntSushi/rust-memchr/blob/master/src/x86/mod.rs#L9-L29
        fn _escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            use std::mem;
            use std::sync::atomic::{AtomicUsize, Ordering};
            use std::fmt::{self, Formatter};
            use std::ptr::addr_of;
            static mut FN: fn(&[u8], &mut Formatter) -> fmt::Result = detect;

            fn detect(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
                let fun = if is_x86_feature_detected!("avx2") {
                    ranges::avx::escape as usize
                } else if is_x86_feature_detected!("sse2") {
                    ranges::sse::escape as usize
                } else {
                    scalar::_escape as usize
                };

                let slot = unsafe { &*(addr_of!(FN) as *const _ as *const AtomicUsize) };
                slot.store(fun, Ordering::Relaxed);
                unsafe {
                    mem::transmute::<usize, fn(&[u8], &mut Formatter) -> fmt::Result>(fun)(
                        bytes, fmt,
                    )
                }
            }

            unsafe {
                let slot = &*(addr_of!(FN) as *const _ as *const AtomicUsize);
                let fun = slot.load(Ordering::Relaxed);
                mem::transmute::<usize, fn(&[u8], &mut Formatter) -> fmt::Result>(fun)(bytes, fmt)
            }
        }

        #[inline(always)]
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        fn _escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            scalar::_escape(bytes, fmt)
        }


        #[inline(always)]
        #[cfg(feature = "bytes-buf")]
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        pub unsafe fn _b_escape<B: buf_min::Buffer>(bytes: &[u8], buf: &mut B) {
            #[cfg(not(v_escape_avx))] {
                #[cfg(not(v_escape_sse))] {
                    scalar::b_escape(bytes, buf)
                }
                #[cfg(v_escape_sse)] {
                    ranges::sse::b_escape(bytes, buf)
                }
            }
            #[cfg(v_escape_avx)] {
                ranges::avx::b_escape(bytes, buf)
            }
        }

        #[inline(always)]
        #[cfg(feature = "bytes-buf")]
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        pub unsafe fn _b_escape<B: buf_min::Buffer>(bytes: &[u8], buf: &mut B) {
            scalar::b_escape(bytes, buf)
        }
    }
}
