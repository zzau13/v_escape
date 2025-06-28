use core::{fmt, str};

use crate::{
    Vector,
    writer::{Writer, write, write_slice},
};

/// A builder trait for creating instances of types that implement the `Escapes` trait.
///
/// # Type Parameters
/// - `V`: The vector type implementing the `Vector` trait.
pub trait EscapesBuilder {
    /// The `Escapes` type for a given vector type.
    ///
    /// # Type Parameters
    /// - `V`: The vector type implementing the `Vector` trait.
    type Escapes<V: Vector>: Escapes<Vector = V>;

    /// Creates a new instance of the `Escapes` type.
    ///
    /// # Returns
    /// An instance of a type that implements the `Escapes` trait.
    fn new<V: Vector>() -> Self::Escapes<V>;
}

/// A trait that abstracts masking functions for escape sequences.
///
/// # Type Parameters
/// - `V`: The vector type implementing the `Vector` trait.
pub trait Escapes: Copy + fmt::Debug {
    /// The length of the escape sequence.
    const ESCAPE_LEN: usize;

    /// Indicates whether the escape sequence may produce false positives.
    const FALSE_POSITIVE: bool;

    /// The vector type used for masking operations.
    type Vector: Vector;

    /// Applies a mask to the given vector `v` to identify escape sequences.
    ///
    /// # Parameters
    /// - `v`: The vector to apply the mask to.
    ///
    /// # Returns
    /// A vector with the mask applied.
    fn masking(&self, v: Self::Vector) -> Self::Vector;

    /// Returns the escape sequence for a given position in the escaped array.
    ///
    /// # Parameters
    /// - `c`: The position of the character.
    ///
    /// # Returns
    /// A static string slice representing the escape sequence.
    fn escape(c: usize) -> &'static str;

    /// Returns the position of a character in the escaped array.
    ///
    /// # Parameters
    /// - `c`: The character to find the position for.
    ///
    /// # Returns
    /// The position of the character.
    fn position(c: u8) -> usize;

    /// Escapes a string by applying escape sequences and writing the result using a writer.
    ///
    /// # Parameters
    /// - `haystack`: The input string to be processed for escape sequences.
    /// - `writer`: A mutable writer function to handle the escaped output.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the escape operation.
    #[inline(always)]
    fn byte_byte_escape<R>(haystack: &str, mut writer: impl Writer<R>) -> Result<(), R> {
        let len = haystack.len();
        let start = haystack.as_ptr();
        unsafe { Self::byte_byte_escape_raw(start, start.add(len), &mut writer) }
    }

    /// Escapes a range of bytes by applying escape sequences and writing the result using a writer.
    ///
    /// # Parameters
    /// - `haystack`: A pointer to the start of the byte range to be escaped.
    /// - `end`: A pointer to the end of the byte range to be escaped.
    /// - `writer`: A mutable writer function to handle the escaped output.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the escape operation.
    ///
    /// # Safety
    /// This function is unsafe because it operates on raw pointers and assumes
    /// that the memory between `haystack` and `end` is valid and properly aligned.
    #[inline(always)]
    unsafe fn byte_byte_escape_raw<R>(
        start: *const u8,
        end: *const u8,
        writer: &mut impl Writer<R>,
    ) -> Result<(), R> {
        unsafe {
            let mut written = start;
            let mut cur = start;
            while cur < end {
                let c = *cur;
                // TODO: improve performance
                if Self::byte_byte_compare(c) {
                    if written < cur {
                        write_slice(written, cur, writer)?;
                    }
                    let escaped = Self::escape(Self::position(c));
                    write(escaped, writer)?;
                    written = cur.add(1);
                }
                cur = cur.add(1);
            }
            if written < end {
                write_slice(written, end, writer)?;
            }
            Ok(())
        }
    }

    /// Compares a byte to determine if it should be escaped.
    ///
    /// # Parameters
    /// - `c`: The byte to compare.
    ///
    /// # Returns
    /// `true` if the byte should be escaped, `false` otherwise.
    fn byte_byte_compare(c: u8) -> bool;
}
