//! A crate for escaping strings
//!
//! # Features
//!
//! - `std`: Enable standard library features
//! - `alloc`: Enable alloc crate features
//! - `string`: Enable `escape_string` function
//! - `fmt`: Enable `escape_fmt` function
//!
//! # Examples
//!
//! ```rust
//! use v_escape_base::{escape_builder, Escapes, EscapesBuilder, Vector};
//!
//! #[derive(Debug, Clone, Copy)]
//! struct Equal<V: Vector> {
//!     a: V,
//! }
//!
//! struct TodoRemoveBuilder;
//! impl EscapesBuilder for TodoRemoveBuilder {
//!     type Escapes<V: Vector> = Equal<V>;
//!
//!     unsafe fn new<V: Vector>() -> Self::Escapes<V> {
//!         Equal { a: V::splat(b'a') }
//!     }
//! }
//!
//! impl<V: Vector> Escapes for Equal<V> {
//!     const ESCAPE_LEN: usize = 1;
//!
//!     const FALSE_POSITIVE: bool = false;
//!
//!     type Vector = V;
//!
//!     #[inline(always)]
//!     unsafe fn masking(&self, vector2: V) -> V {
//!         self.a.cmpeq(vector2)
//!     }
//!
//!     #[inline(always)]
//!     fn escape(_: usize) -> &'static str {
//!         "foo"
//!     }
//!
//!     #[inline(always)]
//!     fn position(_: u8) -> usize {
//!         0
//!     }
//!
//!     #[inline(always)]
//!     fn byte_byte_compare(c: u8) -> bool {
//!         c == b'a'
//!     }
//! }
//!
//! escape_builder!(TodoRemoveBuilder);
//!
//! let mut buffer = String::new();
//! let haystack = "a".repeat(64);
//! # #[cfg(feature = "string")]
//! escape_string(&haystack, &mut buffer);
//! # #[cfg(feature = "string")]
//! assert_eq!(buffer, "foo".repeat(64));
//!
//! let haystack = "a".repeat(64);
//! # #[cfg(feature = "fmt")]
//! assert_eq!(escape_fmt(&haystack).to_string(), "foo".repeat(64));
//! ```
#![deny(missing_docs)]
#![no_std]

/// A module for standard library
#[cfg(any(test, feature = "std"))]
extern crate std;

/// A module for alloc crate
#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

/// A module for architecture-specific escape functions
#[macro_use]
pub mod arch;

/// A module for escapes
mod escapes;

/// A module for extensions
mod ext;

/// A module for generic escape functions
mod generic;
mod vector;
#[macro_use]
mod writer;

pub use escapes::{Escapes, EscapesBuilder};
pub use vector::Vector;
