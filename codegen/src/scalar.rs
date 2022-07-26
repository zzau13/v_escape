use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generator::Tables;
use crate::macros::{escape_body, BodiesArg};
use crate::ranges::Switch;

#[derive(Copy, Clone)]
pub struct ArgScalar<'a> {
    pub ptr: &'a Ident,
    pub start_ptr: &'a Ident,
    pub end_ptr: &'a Ident,
    pub bytes: &'a Ident,
    pub fmt: &'a Ident,
    pub start: &'a Ident,
    pub s: Switch,
}

pub fn escape_scalar(
    ArgScalar {
        ptr,
        start_ptr,
        end_ptr,
        bytes,
        fmt,
        start,
        s,
    }: ArgScalar,
    (t, q, q_len): &Tables,
) -> TokenStream {
    let body = s.fallback_escaping(BodiesArg {
        t,
        q,
        q_len,
        i: &quote! { sub(#ptr, #start_ptr) },
        b: &quote! { *ptr },
        start,
        fmt,
        bytes,
        callback: escape_body,
    });
    quote! {
        while #ptr < #end_ptr {
            #body
            #ptr = #ptr.offset(1);
        }

        #fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..]))?;
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! escape_scalar_bytes {
    ($($t:tt)+) => {
        #[inline]
        pub unsafe fn b_escape<B: $crate::Buffer>(bytes: &[u8], buf: &mut B) {
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();

            let mut ptr = start_ptr;

            let mut start = 0;

            while ptr < end_ptr {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == *ptr {
                            $crate::bodies_exact_one_bytes!(
                                $byte,
                                $quote,
                                (),
                                sub(ptr, start_ptr),
                                *ptr,
                                start,
                                bytes,
                                buf,
                                $crate::escape_body_bytes
                            );
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        $crate::bodies_bytes!(
                            $T,
                            $Q,
                            $Q_LEN,
                            sub(ptr, start_ptr),
                            *ptr,
                            start,
                            bytes,
                            buf,
                            $crate::escape_body_bytes
                        );
                    };
                }

                _inside!(impl $($t)+);

                ptr = ptr.offset(1);
            }

            // Write since start to the end of the slice
            debug_assert!(start <= len);
            if start < len {
                $crate::write_bytes!(&bytes[start..], buf);
            }
        }
    };
}
