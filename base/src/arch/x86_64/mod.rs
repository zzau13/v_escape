/// A module for AVX escape functions
pub mod avx;

/// A module for SSE escape functions
pub mod sse;

/// A macro for creating a escape functions
///
/// # Parameters
/// - `$name`: The name of the function.
/// - `$writer_builder`: The function to use for the builder.
/// - `$builder`: The type of the builder.
/// - `$buffer`: The type of the buffer.
#[doc(hidden)]
#[macro_export]
macro_rules! ifun {
    (
        $name:ident,
        $writer_builder:path,
        $builder:ty,
        $buffer:ty
        $(,$retty:ty)?
    ) => {
        pub fn $name(haystack: &str, buffer: &mut $buffer) $(-> $retty)? {
            use core::sync::atomic::{AtomicPtr, Ordering};

            type Fn = *mut ();
            type RealFn = fn(haystack: &str, buffer: &mut $buffer) $(-> $retty)?;
            static FN: AtomicPtr<()> = AtomicPtr::new(detect as Fn);

            #[cfg(target_feature = "sse2")]
            #[target_feature(enable = "sse2", enable = "avx2")]
            $writer_builder!(escape_avx2, $crate::arch::x86_64::avx::escape, escape, $builder);

            #[cfg(target_feature = "sse2")]
            #[target_feature(enable = "sse2")]
            $writer_builder!(escape_sse2, $crate::arch::x86_64::sse::escape, escape, $builder);

            $writer_builder!(escape_fallback, $crate::arch::fallback::escape_fallback, escape_fallback, $builder);

            unsafe fn detect(haystack: &str, buffer: &mut $buffer) $(-> $retty)? {
                let fun = {
                    #[cfg(not(target_feature = "sse2"))]
                    {
                        escape_fallback
                    }
                    #[cfg(target_feature = "sse2")]
                    {
                        if $crate::arch::x86_64::avx::is_available() {
                            escape_avx2
                        } else if $crate::arch::x86_64::sse::is_available() {
                            escape_sse2
                        } else {
                            escape_fallback
                        }
                    }
                };
                FN.store(fun as Fn, Ordering::Relaxed);
                // SAFETY: The only thing we need to uphold here is the
                // `#[target_feature]` requirements. Since we check is_available
                // above before using the corresponding implementation, we are
                // guaranteed to only call code that is supported on the current
                // CPU.
                fun(haystack, buffer)
            }

            // SAFETY: By virtue of the caller contract, RealFn is a function
            // pointer, which is always safe to transmute with a *mut (). Also,
            // since we use $memchrty::is_available, it is guaranteed to be safe
            // to call $memchrty::$memchrfind.
            unsafe {
                let fun = FN.load(Ordering::Relaxed);
                core::mem::transmute::<Fn, RealFn>(fun)(
                    haystack,
                    buffer
                )
            }
        }
    };
}

/// A macro for creating a escape functions
///
/// # Parameters
/// - `$builder`: The type [`crate::EscapesBuilder`] of the builder
#[macro_export]
macro_rules! escape_builder {
    ($builder:ty) => {
        $crate::struct_display!(
            escape_fmt,
            escape_fmt_internal,
            $crate::ifun!(
                escape_fmt_internal,
                $crate::builder_fmt,
                $builder,
                core::fmt::Formatter<'_>,
                core::fmt::Result
            ),
            $builder
        );

        $crate::struct_string!($crate::ifun!(
            escape_string,
            $crate::builder_string,
            $builder,
            String
        ));

        $crate::struct_bytes!($crate::ifun!(
            escape_bytes,
            $crate::builder_bytes,
            $builder,
            Vec<u8>
        ));
    };
}
