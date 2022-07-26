use crate::ranges::{Feature, Switch};
use crate::utils::ident;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

// TODO: remove
fn sub(a: TokenStream, b: TokenStream) -> TokenStream {
    quote! {
        debug_assert!(#b <= #a);
        (#a as usize) - (#b as usize)
    }
}

fn index(a: &Ident, b: &TokenStream) -> TokenStream {
    quote! {
         unsafe { *#a.as_ptr().add(#b) }
    }
}

#[derive(Copy, Clone)]
pub struct BodyArg<'a> {
    start: &'a Ident,
    fmt: &'a Ident,
    bytes: &'a Ident,
    quote: &'a TokenStream,
}

fn escape_body(
    i: &Ident,
    BodyArg {
        start,
        fmt,
        bytes,
        quote,
    }: BodyArg,
) -> TokenStream {
    quote! {
        if #start < #i {
            #[allow(unused_unsafe)]
            #fmt.write_str(unsafe { std::str::from_utf8_unchecked(&#bytes[#start..#i]) })?;
        }
        #fmt.write_str(#quote)?;
        #start = #i + 1;
    }
}

pub fn mask_body(i: &TokenStream, arg: BodyArg) -> TokenStream {
    let var = &ident("i");
    let body = escape_body(var, arg);
    quote! {
        let #var = #i;
        #body
    }
}

pub trait CB: Fn(&TokenStream, BodyArg) -> TokenStream {}
impl<T: Fn(&TokenStream, BodyArg) -> TokenStream> CB for T {}

pub struct BodiesArg<'a, F: CB> {
    pub t: &'a Ident,
    pub q: &'a Ident,
    pub q_len: &'a Ident,
    pub i: &'a TokenStream,
    pub b: &'a TokenStream,
    pub start: &'a Ident,
    pub fmt: &'a Ident,
    pub bytes: &'a Ident,
    pub callback: F,
}

// TODO: bodies one
pub enum Bodies {
    Reg,
    Exact,
}

pub fn bodies<F: CB>(
    kind: Bodies,
    BodiesArg {
        t,
        q,
        q_len,
        i,
        b,
        start,
        fmt,
        bytes,
        callback,
    }: BodiesArg<F>,
) -> TokenStream {
    let var = &ident("c");
    let ch = &index(t, &quote! { #b as usize });
    let quote = &index(q, &quote! { #var as usize });
    let body = callback(
        i,
        BodyArg {
            start,
            fmt,
            bytes,
            quote,
        },
    );
    match kind {
        Bodies::Reg => quote! {
            let #var = #ch as usize;
            if #var < #q_len {
                #body
            }
        },
        Bodies::Exact => quote! {
            debug_assert_ne!(#ch as usize, #q_len as usize);
            #body
        },
    }
}

fn escape_body_bytes(
    i: &Ident,
    BodyArg {
        start,
        fmt,
        bytes,
        quote,
    }: BodyArg,
) -> TokenStream {
    let write_1 = write_bytes(&quote! { &#bytes[#start..#i] }, fmt);
    let write_2 = write_bytes(&quote! { #quote.as_bytes() }, fmt);
    quote! {
        if #start < #i {
            #write_1
        }
        #write_2

        #start = #i + 1;
    }
}

fn mask_body_bytes(i: &TokenStream, arg: BodyArg) -> TokenStream {
    let var = &ident("i");
    let body = escape_body_bytes(var, arg);
    quote! {
        let #var = #i;
        #body
    }
}

fn write_bytes(bytes: &TokenStream, buf: &Ident) -> TokenStream {
    quote! {
        #buf.extend_from_slice(#bytes);
    }
}
