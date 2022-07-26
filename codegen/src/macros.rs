use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::utils::ident;

fn index(a: &Ident, b: &TokenStream) -> TokenStream {
    quote! { *#a.as_ptr().add(#b) }
}

#[derive(Copy, Clone)]
pub struct BodyArg<'a> {
    start: &'a Ident,
    fmt: &'a Ident,
    bytes: &'a Ident,
    quote: &'a TokenStream,
}

pub fn escape_body(
    i: &TokenStream,
    BodyArg {
        start,
        fmt,
        bytes,
        quote,
    }: BodyArg,
) -> TokenStream {
    quote! {
        let i = #i;
        if #start < i {
            #fmt.write_str(std::str::from_utf8_unchecked(&#bytes[#start..i]))?;
        }
        #fmt.write_str(#quote)?;
        #start = i + 1;
    }
}

pub fn mask_body(i: &TokenStream, arg: BodyArg) -> TokenStream {
    let var = &ident("i");
    let body = escape_body(&quote! { #var }, arg);
    quote! {
        let #var = #i;
        #body
    }
}

pub trait CB: Fn(&TokenStream, BodyArg) -> TokenStream + Copy {}
impl<T: Fn(&TokenStream, BodyArg) -> TokenStream + Copy> CB for T {}

#[derive(Copy, Clone)]
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

pub fn escape_body_bytes(
    i: &TokenStream,
    BodyArg {
        start,
        fmt,
        bytes,
        quote,
    }: BodyArg,
) -> TokenStream {
    let var = &ident("i");
    let write_1 = write_bytes(&quote! { &#bytes[#start..#var] }, fmt);
    let write_2 = write_bytes(&quote! { ( #quote ).as_bytes() }, fmt);
    quote! {
        let #var = #i;
        if #start < #var {
            #write_1
        }
        #write_2

        #start = #var + 1;
    }
}

pub fn mask_body_bytes(i: &TokenStream, arg: BodyArg) -> TokenStream {
    let var = &ident("i");
    let body = escape_body_bytes(&var.to_token_stream(), arg);
    quote! {
        let #var = #i;
        #body
    }
}

pub fn write_bytes(bytes: &TokenStream, buf: &Ident) -> TokenStream {
    quote! {
        #buf.extend_from_slice(#bytes);
    }
}
