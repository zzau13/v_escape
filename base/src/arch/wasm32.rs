use core::arch::wasm32::v128;

use crate::{Escapes, EscapesBuilder, Vector, generic::Generic, writer::Writer};

type WasmVector = v128;
/// A function that performs escape operations using Wasm SIMD vectorization.
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
    if len < WasmVector::BYTES {
        return <E::Escapes<()> as Escapes>::byte_byte_escape(haystack, writer);
    }

    // # Safety
    // E::new::<v128>() is unsafe because it operates simd instructions.
    Generic::new(E::new::<WasmVector>()).escape(haystack, writer)
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
            $crate::arch::wasm32::escape,
            escape,
            $builder
        ));

        $crate::struct_display!(
            escape_fmt,
            escape_fmt_internal,
            $crate::builder_fmt!(
                escape_fmt_internal,
                $crate::arch::wasm32::escape,
                escape,
                $builder
            ),
            $builder
        );
    };
}
