use core::arch::x86_64::__m128i;

use crate::{Escapes, EscapesBuilder, Vector, generic::Generic, writer::Writer};

/// Returns true if SSE2 is available in the current environment.
pub fn is_available() -> bool {
    #[cfg(feature = "std")]
    {
        std::is_x86_feature_detected!("sse2")
    }
    #[cfg(not(feature = "std"))]
    {
        false
    }
}

type SseVector = __m128i;

/// A function that performs escape operations using SSE vectorization.
///
/// # Parameters
/// - `haystack`: The input string to be escaped.
/// - `writer`: The writer function.
///
/// # Returns
/// A result indicating success or failure of the escape operation.
#[inline(always)]
pub fn escape<E: EscapesBuilder, R>(haystack: &str, writer: impl Writer<R>) -> Result<(), R> {
    let len = haystack.len();
    if len < SseVector::BYTES {
        return <E::Escapes<()> as Escapes>::byte_byte_escape(haystack, writer);
    }

    // # Safety
    // E::new::<__m128i>() is unsafe because it operates simd instructions.
    Generic::new(unsafe { E::new::<SseVector>() }).escape(haystack, writer)
}
