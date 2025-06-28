use core::arch::x86_64::{__m128i, __m256i};

use crate::{Escapes, EscapesBuilder, Vector, generic::Generic, writer::Writer};

// Adapted from https://github.com/BurntSushi/memchr/blob/master/src/arch/x86_64/avx2/memchr.rs
/// Returns true if AVX2 is available in the current environment.
pub fn is_available() -> bool {
    #[cfg(not(target_feature = "sse2"))]
    {
        false
    }
    #[cfg(target_feature = "sse2")]
    {
        #[cfg(target_feature = "avx2")]
        {
            true
        }
        #[cfg(not(target_feature = "avx2"))]
        {
            #[cfg(feature = "std")]
            {
                std::is_x86_feature_detected!("avx2")
            }
            #[cfg(not(feature = "std"))]
            {
                false
            }
        }
    }
}

type AvxVector = __m256i;
type SseVector = __m128i;

/// A function that performs escape operations using AVX and SSE vectorization.
///
/// # Parameters
/// - `haystack`: The input string to be escaped.
/// - `writer`: The writer function.
///
/// # Returns
/// A result indicating success or failure of the escape operation.
#[inline(always)]
pub fn escape<E: EscapesBuilder, R>(haystack: &str, mut writer: impl Writer<R>) -> Result<(), R> {
    let len = haystack.len();
    if len < AvxVector::BYTES {
        if len < SseVector::BYTES {
            return <E::Escapes<()> as Escapes>::byte_byte_escape(haystack, &mut writer);
        }
        // # Safety
        // E::new::<__m128i>() is unsafe because it operates simd instructions.
        return Generic::new(E::new::<SseVector>()).escape(haystack, writer);
    }

    // # Safety
    // E::new::<__m256i>() is unsafe because it operates simd instructions.
    Generic::new(E::new::<__m256i>()).escape(haystack, writer)
}
