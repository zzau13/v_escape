#![allow(unused)]
#![deny(missing_docs)]

//! Code generation base library for v_escape
//!
//! This crate provides the core functionality for generating SIMD-optimized escape functions
//! from character mappings. It parses template files and generates the necessary code
//! for efficient string escaping operations.

use generator::{Generator, parse_template};
use proc_macro2::{Span, TokenStream};

mod generator;
mod pairs;
mod switch;

/// Generate escape functions from a token stream template
///
/// This function takes a token stream representing character mappings and generates
/// the corresponding escape function implementations. It returns both the generated
/// code and metadata about the escape mappings.
///
/// # Arguments
///
/// * `tokens` - A token stream containing the character mappings in the format `new!(char -> "escape", ...)`
/// * `crate_name` - The name of the crate where the generated code will be used
///
/// # Returns
///
/// Returns a tuple containing:
/// * The generated code as a `TokenStream`
/// * A tuple of `(escapes, escaped)` where `escapes` is a string of characters to escape
///   and `escaped` is the concatenated escape sequences
///
/// # Errors
///
/// Returns a `syn::Error` if the token stream cannot be parsed or if the character
/// mappings are invalid.
pub fn generate(
    tokens: TokenStream,
    crate_name: &str,
) -> syn::Result<(TokenStream, (String, String))> {
    let pairs = parse_template(tokens)?;
    let generator = Generator::new(&pairs, crate_name);
    let generated = generator.build();
    let (escapes, escaped): (Vec<u8>, Vec<String>) =
        pairs.into_iter().map(|p| (p.ch, p.quote)).unzip();
    let escapes = String::from_utf8(escapes)
        .map_err(|e| syn::Error::new(Span::call_site(), e.to_string()))?;
    let escaped = escaped.join("");

    Ok((generated, (escapes, escaped)))
}
