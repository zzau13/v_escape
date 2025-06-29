use core::arch::aarch64::int8x16_t;

use crate::{Escapes, EscapesBuilder, Vector, generic::Generic, writer::Writer};

type NeonVector = int8x16_t;

/// A function that performs escape operations using NEON SIMD vectorization.
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
    if len < NeonVector::BYTES {
        return <E::Escapes<()> as Escapes>::byte_byte_escape(haystack, writer);
    }

    // # Safety
    // E::new::<int8x16_t>() is unsafe because it operates simd instructions.
    Generic::new(E::new::<NeonVector>()).escape(haystack, writer)
}

/// A macro for creating a escape functions
///
/// # Parameters
/// - `$builder`: The type [`crate::EscapesBuilder`] of the builder
#[macro_export]
macro_rules! escape_builder {
    ($builder:ty) => {
        $crate::struct_string!($crate::builder_string!(
            escape_string,
            $crate::arch::aarch64::escape,
            escape,
            $builder
        ));
        $crate::struct_display!(
            escape_fmt,
            escape_fmt_internal,
            $crate::builder_fmt!(
                escape_fmt_internal,
                $crate::arch::aarch64::escape,
                escape,
                $builder
            ),
            $builder
        );
    };
}
