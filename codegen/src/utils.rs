use proc_macro2::{Ident, Span};

pub fn ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

pub const BUF_MIN_VERSION: &str = "0.7";
