use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generator::Tables;
use crate::macros::{escape_body, escape_body_bytes, write_bytes, BodiesArg};
use crate::ranges::Switch;

#[derive(Copy, Clone)]
pub struct ArgScalar<'a> {
    pub ptr: &'a Ident,
    pub start_ptr: &'a Ident,
    pub end_ptr: &'a Ident,
    pub bytes: &'a Ident,
    pub fmt: &'a Ident,
    pub len: &'a Ident,
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
        ..
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

pub fn escape_scalar_bytes(
    ArgScalar {
        ptr,
        start_ptr,
        end_ptr,
        bytes,
        fmt,
        start,
        len,
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
        callback: escape_body_bytes,
    });
    let write_1 = write_bytes(&quote! { &#bytes[#start..] }, fmt);
    quote! {
        while #ptr < #end_ptr {
            #body
            #ptr = #ptr.offset(1);
        }

        debug_assert!(#start <= #len);
        if #start < #len {
            #write_1
        }

    }
}
