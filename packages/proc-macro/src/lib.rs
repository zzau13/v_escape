#![deny(missing_docs)]

//! Procedural macro crate for v_escape
//!
//! This crate provides the `escape!` procedural macro for generating SIMD-optimized
//! escape functions from character mappings. The macro generates both `escape_string`
//! and `escape_fmt` functions that can be used for efficient string escaping.

use proc_macro2::TokenStream;
use quote::quote;
use v_escape_codegen_base::generate;

/// Generate escape functions from a list of character mappings.
///
/// This procedural macro generates SIMD-optimized escape functions for efficiently
/// replacing specific characters with their corresponding escape sequences. The macro
/// creates both `escape_string` and `escape_fmt` functions that can be used to escape
/// strings in various contexts (HTML, JSON, LaTeX, etc.).
///
/// # Syntax
///
/// ```rust,ignore
/// escape! {
///     character -> "escape_sequence",
///     character -> "escape_sequence",
///     // ... more mappings
/// }
/// ```
///
/// # Parameters
///
/// - `character`: A character to be escaped, specified as:
///   - A byte literal: `b'"'`, `b'<'`, `b'&'`
///   - A char literal: `'"'`, `'<'`, `'&'`
///   - An integer literal: `34`, `60`, `38` (ASCII values)
/// - `escape_sequence`: A string literal containing the replacement text
///
/// # Generated Functions
///
/// The macro generates the following functions in the current module:
///
/// - `escape_string(input: &str, buffer: &mut String)`: Escapes the input string and
///   appends the result to the provided buffer
/// - `escape_fmt(input: &str) -> impl std::fmt::Display`: Returns a displayable object
///   that formats the escaped string
///
/// # Features
///
/// The generated functions require specific features to be enabled:
/// - `string` feature: Enables `escape_string` function
/// - `fmt` feature: Enables `escape_fmt` function
///
/// # Performance
///
/// The generated functions use SIMD vectorization for optimal performance:
/// - Automatically selects the best available SIMD instructions (AVX2, SSE2, NEON, WASM, etc.)
/// - Falls back to scalar operations when SIMD is not available
/// - Uses efficient lookup tables for character matching
/// - Minimizes memory allocations and copies
///
/// # Error Handling
///
/// The macro will fail to compile if:
/// - Character mappings are not properly formatted
/// - Duplicate characters are specified
/// - Non-ASCII characters are used (only ASCII characters 0-127 are supported)
/// - Escape sequences are not valid string literals
///
/// # Implementation Details
///
/// The macro generates:
/// - Static lookup tables for efficient character matching
/// - SIMD-optimized escape implementations for various architectures
/// - Fallback implementations for compatibility
/// - Proper trait implementations for the v_escape framework
#[proc_macro]
pub fn escape(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    match generate(quote!(new!( #input );), "v_escape") {
        Ok((code, _)) => code.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
