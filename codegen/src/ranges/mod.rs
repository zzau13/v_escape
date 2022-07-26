use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub use self::switch::Switch;
use crate::utils::ident;

mod avx;
mod sse;
mod switch;

#[derive(Copy, Clone)]
pub enum Feature {
    Avx2,
    Sse2,
}
use Feature::*;

pub trait WriteMask: Fn(&Ident, &Ident) -> TokenStream + Copy {}
impl<T: Fn(&Ident, &Ident) -> TokenStream + Copy> WriteMask for T {}

#[derive(Copy, Clone)]
pub struct ArgLoop<'a, WM: WriteMask, WF: WriteMask> {
    len: &'a Ident,
    ptr: &'a Ident,
    start_ptr: &'a Ident,
    end_ptr: &'a Ident,
    s: Switch,
    write_mask: WM,
    write_forward: WF,
}

impl Feature {
    fn to_impl<WM: WriteMask, WF: WriteMask>(
        self,
        arg: ArgLoop<WM, WF>,
    ) -> (&'static str, TokenStream) {
        match self {
            Avx2 => ("avx2", avx::loop_avx2(arg)),
            Sse2 => ("sse2", sse::loop_sse(arg)),
        }
    }
}

pub fn escape_range(s: Switch, f: Feature) -> TokenStream {
    let len = &ident("len");
    let start_ptr = &ident("start_ptr");
    let end_ptr = &ident("end_ptr");
    let ptr = &ident("ptr");
    let arg = ArgLoop {
        len,
        ptr,
        end_ptr,
        start_ptr,
        s,
        write_mask: |mask: &Ident, ptr: &Ident| {
            // TODO
            quote! {
                let at = crate::sub!(#ptr, #start_ptr);
                let mut cur = #mask.trailing_zeros() as usize;

                loop {
                    mask_bodies!(#mask, at, cur, #ptr);
                }

                debug_assert_eq!(at, #ptr - #start_ptr)
            }
        },
        // TODO
        write_forward: |mask: &Ident, align: &Ident| {
            quote! {
                let at = crate::sub!(#ptr, #start_ptr);
                let mut cur = mask.trailing_zeros() as usize;

                while cur < #align {
                    mask_bodies!($mask, at, cur, #ptr);
                }

                debug_assert_eq!(at, #ptr - #start_ptr)
            }
        },
    };
    let (feature, loops) = f.to_impl(arg);

    quote! {
        #[target_feature(enable = #feature)]
        pub unsafe fn escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::*;
            #[cfg(target_arch = "x86")]
            use std::arch::x86::*;

            let #len = bytes.len();
            let #start_ptr = bytes.as_ptr();
            let #end_ptr = bytes[len..].as_ptr();
            let mut #ptr = #start_ptr;
            let mut start = 0;

            #loops
            debug_assert!(start <= #len);
            if start < #len {
                fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..#len]))?;
            }

            Ok(())
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! escape_ranges_bytes {
    (avx2 $($t:tt)+) => {
        #[inline]
        #[target_feature(enable = "avx2")]
        $crate::escape_ranges_bytes!(impl $crate::loop_range_switch_avx2 where $($t)+);
    };
    (sse2 $($t:tt)+) => {
        #[inline]
        #[target_feature(enable = "sse2")]
        $crate::escape_ranges_bytes!(impl $crate::loop_range_switch_sse2 where $($t)+);
    };
    (impl $loops:path where ($T:ident, $Q:ident, $Q_LEN:ident) $($t:tt)+) => {
        pub unsafe fn b_escape<B: $crate::Buffer>(bytes: &[u8], buf: &mut B) {
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;

            let mut start = 0;

            macro_rules! mask_bodies_callback {
                ($callback:path) => {
                    // Format bytes in the mask that starts in the current pointer
                    macro_rules! mask_bodies {
                        ($mask:ident, $at:ident, $cur:ident, $ptr:ident) => {
                            // Calls macro `bodies!` at position `$at + $cur`
                            // of byte `*$ptr` + `$curr` with macro `$crate::mask_body!`
                            $callback!($T, $Q, $Q_LEN, $at + $cur, *$ptr.add($cur), start, bytes, buf, $crate::mask_body_bytes);

                            // Create binary vector of all zeros except
                            // position `$curr` and xor operation with `$mask`
                            $mask ^= 1 << $cur;
                            // Test vs Check  if `$mask` is empty
                            if $mask == 0 {
                                break;
                            }

                            // Get to the next possible escape character avoiding zeros
                            $cur = $mask.trailing_zeros() as usize;
                        };
                    }
                };
            }

            $crate::mask_bodies_escaping_bytes!($($t)+);

            // Macro to write with mask
            macro_rules! write_mask {
                ($mask:ident, $ptr:ident) => {{
                    // Reference to the start of mask
                    let at = $crate::sub!($ptr, start_ptr);
                    // Get to the first possible escape character avoiding zeros
                    let mut cur = $mask.trailing_zeros() as usize;

                    loop {
                        // Writing in `$fmt` with `$mask`
                        // The main loop will break when mask == 0
                        mask_bodies!($mask, at, cur, $ptr);
                    }

                    debug_assert_eq!(at, $crate::sub!($ptr, start_ptr))
                }};
            }

            // Write a sliced mask
            macro_rules! write_forward {
                ($mask: ident, $align:ident) => {{
                    let at = $crate::sub!(ptr, start_ptr);
                    let mut cur = $mask.trailing_zeros() as usize;

                    while cur < $align {
                        mask_bodies!($mask, at, cur, ptr);
                    }

                    debug_assert_eq!(at, $crate::sub!(ptr, start_ptr))
                }};
            }

            macro_rules! fallback_callback {
                (default) => {
                    macro_rules! fallback {
                        () => {
                            while ptr < end_ptr {
                                $crate::bodies_bytes!(
                                    $T,
                                    $Q,
                                    $Q_LEN,
                                    $crate::sub!(ptr, start_ptr),
                                    *ptr,
                                    start,
                                    bytes,
                                    buf,
                                    $crate::mask_body_bytes
                                );
                                ptr = ptr.offset(1);
                            }
                        };
                    }
                };
                (one) => {
                    macro_rules! fallback {
                        () => {
                            while ptr < end_ptr {
                                if *ptr == $T {
                                    $crate::bodies_exact_one_bytes!(
                                        $T,
                                        $Q,
                                        $Q_LEN,
                                        $crate::sub!(ptr, start_ptr),
                                        *ptr,
                                        start,
                                        bytes,
                                        buf,
                                        $crate::mask_body_bytes
                                    );
                                }
                                ptr = ptr.offset(1);
                            }
                        };
                    }
                };
            }

            $crate::fallback_escaping!($($t)+);

            $loops!((len, ptr, start_ptr, end_ptr) $($t)+);

            // Write since start to the end of the slice
            debug_assert!(start <= len);
            if start < #len {
                $crate::write_bytes!(&bytes[start..], buf);
            }
        }
    };
}
