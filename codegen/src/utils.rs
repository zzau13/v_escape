use proc_macro2::{Ident, Span};

pub fn ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}
