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
use crate::generator::Tables;
use crate::macros::{bodies, mask_body, BodiesArg};
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

pub fn escape_range(s: Switch, (t, q, q_len): &Tables, f: Feature) -> TokenStream {
    let len = &ident("len");
    let start_ptr = &ident("start_ptr");
    let end_ptr = &ident("end_ptr");
    let ptr = &ident("ptr");
    let at = &ident("at");
    let cur = &ident("cur");
    let start = &ident("start");
    let fmt = &ident("fmt");
    let bytes = &ident("bytes");
    let body = bodies(
        s.into(),
        BodiesArg {
            t,
            q,
            q_len,
            i: &quote! { #at + #cur },
            b: &quote! { *#ptr.add(#cur) },
            start,
            fmt,
            bytes,
            callback: mask_body,
        },
    );
    let mask_bodies = |mask: &Ident| {
        quote! {
            #body
            #mask ^= 1 << #cur;
            if #mask == 0 {
                break;
            }
            #cur = #mask.trailing_zeros() as usize;
        }
    };
    let arg = ArgLoop {
        len,
        ptr,
        end_ptr,
        start_ptr,
        s,
        write_mask: |mask: &Ident, ptr: &Ident| {
            let body = mask_bodies(mask);
            quote! {
                let #at = crate::sub!(#ptr, #start_ptr);
                let mut #cur = #mask.trailing_zeros() as usize;

                loop {
                    #body
                }

                debug_assert_eq!(#at, #ptr - #start_ptr)
            }
        },
        write_forward: |mask: &Ident, align: &Ident| {
            let body = mask_bodies(mask);
            quote! {
                let #at = crate::sub!(#ptr, #start_ptr);
                let mut #cur = #mask.trailing_zeros() as usize;

                while #cur < #align {
                    #body
                }

                debug_assert_eq!(#at, #ptr - #start_ptr)
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
            let mut #start = 0;

            #loops
            debug_assert!(start <= #len);
            if #start < #len {
                fmt.write_str(std::str::from_utf8_unchecked(&bytes[#start..#len]))?;
            }

            Ok(())
        }
    }
}
