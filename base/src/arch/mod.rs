/// A module for x86_64 escape functions
#[cfg(all(
    target_arch = "x86_64",
    any(target_feature = "sse2", target_feature = "avx2")
))]
#[macro_use]
pub mod x86_64;

/// A module for aarch64 escape functions
#[cfg(target_arch = "aarch64")]
#[macro_use]
pub mod aarch64;

/// A module for wasm32 escape functions
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[macro_use]
pub mod wasm32;

/// A module for fallback escape functions
pub mod fallback;

/// A macro for creating a escape functions
///
/// # Parameters
/// - `$builder`: The type [`crate::EscapesBuilder`] of the builder
#[cfg(not(any(
    all(
        target_arch = "x86_64",
        any(target_feature = "sse2", target_feature = "avx2")
    ),
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
)))]
#[macro_export]
macro_rules! escape_builder {
    ($builder:ty) => {
        $crate::struct_string!($crate::builder_string!(
            escape_string,
            $crate::arch::fallback::escape_fallback,
            escape_fallback,
            $builder
        ));

        $crate::struct_display!(
            escape_fmt,
            escape_fmt_internal,
            $crate::builder_fmt!(
                escape_fmt_internal,
                $crate::arch::fallback::escape_fallback,
                escape_fallback,
                $builder
            ),
            $builder
        );
    };
}
