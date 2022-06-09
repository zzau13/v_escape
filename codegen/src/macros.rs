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
struct BodyArg<'a> {
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

fn mask_body(i: &TokenStream, arg: BodyArg) -> TokenStream {
    let var = &ident("i");
    let body = escape_body(var, arg);
    quote! {
        let #var = #i;
        #body
    }
}

struct BodiesArg<'a> {
    t: &'a Ident,
    q: &'a Ident,
    q_len: &'a Ident,
    i: &'a TokenStream,
    b: &'a TokenStream,
    start: &'a Ident,
    fmt: &'a Ident,
    bytes: &'a Ident,
    callback: Box<dyn Fn(&TokenStream, BodyArg) -> TokenStream>,
}

// TODO: bodies one
enum Bodies {
    Reg,
    Exact,
}

fn bodies(
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
    }: BodiesArg,
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
    quote! {
        if #start < #i {
            crate::write_bytes!(&#bytes[#start..#i], #fmt);
        }
        crate::write_bytes!(#quote.as_bytes(), #fmt);

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

macro_rules! write_bytes {
    ($bytes:expr, $buf:ident) => {
        $buf.extend_from_slice($bytes);
    };
}

macro_rules! bodies_bytes {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $bytes:ident, $buf:ident, $callback:path) => {
        let c = $crate::index!($T[$b as usize]) as usize;
        if c < $Q_LEN {
            $callback!($i, $start, $bytes, $buf, $crate::index!($Q[c]));
        }
    };
}
macro_rules! bodies_exact_bytes {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $bytes:ident, $buf:ident, $callback:path) => {
        debug_assert_ne!($T[$b as usize] as usize, $Q_LEN as usize);
        $callback!(
            $i,
            $start,
            $bytes,
            $buf,
            $crate::index!($Q[$crate::index!($T[$b as usize]) as usize])
        );
    };
}
macro_rules! bodies_exact_one_bytes {
    ($char:expr, $quote:expr, $_non:expr, $i:expr, $b:expr, $start:ident, $bytes:ident, $buf:ident, $callback:path) => {
        debug_assert_eq!($char, $b);
        $callback!($i, $start, $bytes, $buf, $quote);
    };
}
