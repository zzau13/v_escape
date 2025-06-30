use crate::{Escapes, EscapesBuilder, writer::Writer};

/// A function that performs escape operations using fallback implementation.
///
/// # Parameters
/// - `haystack`: The input string to be escaped.
/// - `writer`: The writer function.
///
/// # Returns
/// A result indicating success or failure of the escape operation.
#[inline(always)]
pub fn escape_fallback<E: EscapesBuilder, R>(
    haystack: &str,
    writer: impl Writer<R>,
) -> Result<(), R> {
    // TODO: implement "1.21 Scanning for zero bytes" from Matters Computational by J. Arndt
    // https://www.researchgate.net/publication/267072412_Matters_Computational_Ideas_Algorithms_Source_Code
    // Is not possible with range of bytes, not exist operations for this or are very expensive
    // So we need to use different approach
    // But this is fallback implementation, so it's not priority
    E::Escapes::<()>::byte_byte_escape(haystack, writer)
}
