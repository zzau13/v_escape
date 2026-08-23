use crate::ext::Pointer;
use core::{fmt, result::Result as BResult, slice, str};
use derive_new::new;

// TODO: Maybe Writer<T: Add<Output = T>, R> = FnMut(&str) -> Result<T, R>
// struct Foo;
// impl core::ops::Add for Foo {
//     type Output = Foo;
//     #[inline(always)]
//     fn add(self, _rhs: Self) -> Self::Output {
//         self
//     }
// }
// impl From<Foo> for () {
//     #[inline(always)]
//     fn from(_: Foo) -> Self {
//         ()
//     }
// }
// And implement for &mut dyn core::fmt::Write and &mut dyn core::io::Write

/// A trait for writing strings, defined as a function that takes a string slice
/// and returns a `Result`.
///
/// # Type Parameters
/// - `R`: The result type for the writer function.
///
pub(crate) type Result<Err> = BResult<(), Err>;

/// Sink abstraction that escape routines append output to.
pub trait Writer {
    /// Error type returned by [`Writer::write_str`].
    type Error;

    /// Appends the contents of `src` to the writer.
    fn write_slice(&mut self, src: &[u8]) -> Result<Self::Error>;

    /// Appends the UTF-8 bytes between `start` and `end` to the writer.
    /// # SAFETY
    /// This function is only used internally
    /// Not use this trait
    unsafe fn write_ptr(&mut self, start: *const u8, end: *const u8) -> Result<Self::Error> {
        debug_assert!(start < end);
        // # SAFETY
        // Internal trait: Intensive testing and debug check for correctness
        let src = unsafe {
            alloc::slice::from_raw_parts(start, (end as usize).wrapping_sub(start as usize))
        };
        self.write_slice(src)
    }

    /// Appends the contents of `src` to the writer.
    fn write_str(&mut self, src: &str) -> Result<Self::Error> {
        self.write_slice(src.as_bytes())
    }
}

/// [`Writer`] implementation that appends bytes to a borrowed [`alloc::vec::Vec`].
#[cfg(feature = "alloc")]
#[repr(transparent)]
#[derive(new)]
pub struct WriterVec<'a> {
    inner: &'a mut alloc::vec::Vec<u8>,
}

#[cfg(feature = "alloc")]
impl Writer for WriterVec<'_> {
    type Error = ();

    fn write_slice(&mut self, src: &[u8]) -> Result<Self::Error> {
        self.inner.extend_from_slice(src);
        Ok(())
    }
}

/// [`Writer`] implementation that forwards bytes to a [`core::fmt::Formatter`].
#[cfg(feature = "fmt")]
#[repr(transparent)]
#[derive(new)]
pub struct WriterFMT<'a, 'b> {
    inner: &'a mut core::fmt::Formatter<'b>,
}

#[cfg(feature = "fmt")]
impl Writer for WriterFMT<'_, '_> {
    type Error = fmt::Error;

    fn write_slice(&mut self, src: &[u8]) -> Result<Self::Error> {
        // SAFETY: Internal trait, only used for valid UTF-8 slices.
        unsafe { self.write_str(str::from_utf8_unchecked(src)) }
    }

    #[inline(always)]
    fn write_str(&mut self, src: &str) -> Result<Self::Error> {
        self.inner.write_str(src)
    }
}

/// Writes a string slice using the writer function.
///
/// # Parameters
/// - `src`: The string slice to be written.
/// - `writer`: The function to write the string slice.
///
/// # Returns
/// A `Result` indicating the success or failure of the write operation.
#[inline(always)]
pub(crate) fn write<W: Writer>(src: &str, writer: &mut W) -> Result<W::Error> {
    writer.write_str(src)
}

/// Writes a slice of bytes as a string using the writer function.
///
/// # Parameters
/// - `start`: The starting pointer of the byte slice.
/// - `end`: The ending pointer of the byte slice.
/// - `writer`: The function to write the string slice.
///
/// # Returns
/// A `Result` indicating the success or failure of the write operation.
///
/// # Safety
/// This function is unsafe because it assumes that the byte slice is valid UTF-8.
#[inline(always)]
pub(crate) unsafe fn write_slice<W: Writer>(
    start: *const u8,
    end: *const u8,
    writer: &mut W,
) -> Result<W::Error> {
    unsafe {
        write(
            str::from_utf8_unchecked(slice::from_raw_parts(start, end.distance(start))),
            writer,
        )
    }
}

/// A macro for creating a builder function that appends a string to a `String`.
/// implement: `writer_builder!(Function Name, Function Path, Function External Name, Builder Struct Type, ...Attributes)`.
///
/// # Parameters
/// - `$name`: The name of the builder function.
/// - `$fn`: The function to use for the builder.
/// - `$fn_name`: The name of the function to use for the builder.
/// - `$builder`: The type of the builder.
/// - `$($attr:tt)`: The attributes to add to the function.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "fmt")]
macro_rules! builder_fmt {
    ($name:ident, $fn:path, $fn_name:ident, $builder:ty) => {
        $crate::builder_fmt!($name, $fn, $fn_name, $builder,);
    };
    ($name:ident, $fn:path, $fn_name:ident, $builder:ty, $($attr:tt)*) => {
        $($attr)*
        fn $name<'a>(haystack: &str, buffer: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            use $fn;
            let writer = $crate::writer::WriterFMT::new(buffer);
            $fn_name::<$builder, _>(haystack, writer)
        }
    };
}

/// A macro for creating a function that return a `impl Display`.
///
/// # Parameters
/// - `$name`: The name of the function.
/// - `$internal`: The internal function to use for the struct.
/// - `$body`: The body of the struct.
/// - `$builder`: The type of the builder.
/// - `$($attr:tt)`: The attributes to add to the function.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "fmt")]
macro_rules! struct_display {
    ($name:ident, $internal:ident, $body:expr, $builder:ty) => {
        /// Returns a value implementing [`core::fmt::Display`] that escapes
        /// `haystack` lazily into the formatter it is rendered to.
        ///
        /// The returned value borrows from `haystack` and only performs work when
        /// it is actually formatted (e.g. via `format!`, `println!`, or
        /// `core::fmt::Write`). At runtime the best available SIMD backend is
        /// selected once per call site; on platforms without SIMD support a
        /// scalar fallback is used.
        ///
        /// See the crate-level documentation for the table of characters that
        /// get rewritten and their replacements.
        pub fn $name<'a>(haystack: &'a str) -> impl core::fmt::Display + 'a {
            struct __SDisplay<'a>(&'a str);

            impl<'a> core::fmt::Display for __SDisplay<'a> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    $body;
                    $internal(self.0, f)
                }
            }

            __SDisplay(haystack)
        }
    };
}

#[cfg(not(feature = "fmt"))]
#[macro_export]
#[doc(hidden)]
macro_rules! struct_display {
    ($($tt:tt)*) => {};
}

/// A macro for creating a builder function that appends a string to a `String`.
/// implement: `writer_builder!(Function Name, Function Path, Function External Name, Builder Struct Type, ...Attributes)`.
///
/// # Parameters
/// - `$name`: The name of the builder function.
/// - `$fn`: The function to use for the builder.
/// - `$fn_name`: The name of the function to use for the builder.
/// - `$builder`: The type of the builder.
/// - `$($attr:tt)`: The attributes to add to the function.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "string")]
macro_rules! builder_string {
    ($name:ident, $fn:path, $fn_name:ident, $builder:ty) => {
        $crate::builder_string!($name, $fn, $fn_name, $builder,);
    };
    ($name:ident, $fn:path, $fn_name:ident, $builder:ty, $($attr:tt)*) => {
        /// Escapes `haystack` and appends the result to `buffer`.
        ///
        /// Bytes that are not part of the escape table defined for this crate
        /// are forwarded verbatim. The function never clears `buffer`; callers
        /// that want a fresh result should pass an empty `String`.
        ///
        /// See the crate-level documentation for the table of characters that
        /// get rewritten and their replacements.
        $($attr)*
        pub fn $name(haystack: &str, buffer: &mut String) {
            use $fn;
            // SAFETY: The escape routine only writes valid UTF-8: the input
            // `haystack` is forwarded verbatim and every replacement emitted
            // through the escape table is itself valid UTF-8. Therefore the
            // `String` invariant is upheld after the writer is dropped.
            let vec = unsafe { buffer.as_mut_vec() };
            let writer = $crate::writer::WriterVec::new(vec);

            let _ = $fn_name::<$builder, _>(haystack, writer);
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "string")]
macro_rules! struct_string {
    ($($tt:tt)*) => {
        $($tt)*;
    };
}

#[cfg(not(feature = "string"))]
#[macro_export]
#[doc(hidden)]
macro_rules! struct_string {
    ($($tt:tt)*) => {};
}

/// A macro for creating a builder function that appends a Vector to a `Vector`.
/// implement: `writer_builder!(Function Name, Function Path, Function External Name, Builder Struct Type, ...Attributes)`.
///
/// # Parameters
/// - `$name`: The name of the builder function.
/// - `$fn`: The function to use for the builder.
/// - `$fn_name`: The name of the function to use for the builder.
/// - `$builder`: The type of the builder.
/// - `$($attr:tt)`: The attributes to add to the function.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "bytes")]
macro_rules! builder_bytes {
    ($name:ident, $fn:path, $fn_name:ident, $builder:ty) => {
        $crate::builder_bytes!($name, $fn, $fn_name, $builder,);
    };
    ($name:ident, $fn:path, $fn_name:ident, $builder:ty, $($attr:tt)*) => {
        /// Escapes `haystack` and appends the resulting UTF-8 bytes to `buffer`.
        ///
        /// `haystack` is required to be a valid `&str` so the escaped output is
        /// itself guaranteed to be valid UTF-8. The function never clears
        /// `buffer`; callers that want a fresh result should pass an empty
        /// `Vec`.
        ///
        /// See the crate-level documentation for the table of characters that
        /// get rewritten and their replacements.
        $($attr)*
        pub fn $name(haystack: &str, buffer: &mut Vec<u8>) {
            use $fn;
            let writer = $crate::writer::WriterVec::new(buffer);
            let _ = $fn_name::<$builder, _>(haystack, writer);
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "bytes")]
macro_rules! struct_bytes {
    ($($tt:tt)*) => {
        $($tt)*;
    };
}

#[cfg(not(feature = "bytes"))]
#[macro_export]
#[doc(hidden)]
macro_rules! struct_bytes {
    ($($tt:tt)*) => {};
}
