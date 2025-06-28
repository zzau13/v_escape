#![allow(unused)]

use generator::{Generator, parse_template};
use proc_macro2::{Span, TokenStream};

mod generator;
mod pairs;
mod switch;

pub fn generate(tokens: TokenStream) -> syn::Result<(TokenStream, (String, String))> {
    let pairs = parse_template(tokens)?;
    let generator = Generator::new(&pairs);
    let generated = generator.build();
    let (escapes, escaped): (Vec<u8>, Vec<String>) =
        pairs.into_iter().map(|p| (p.ch, p.quote)).unzip();
    let escapes = String::from_utf8(escapes)
        .map_err(|e| syn::Error::new(Span::call_site(), e.to_string()))?;
    let escaped = escaped.join("");

    Ok((generated, (escapes, escaped)))
}
