use core::{slice, str};

use crate::ext::Pointer;

/// A trait for writing strings, defined as a function that takes a string slice
/// and returns a `Result`.
///
/// # Type Parameters
/// - `R`: The result type for the writer function.
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
pub trait Writer<R>: FnMut(&str) -> Result<(), R> {}

impl<R, T: FnMut(&str) -> Result<(), R>> Writer<R> for T {}

/// Writes a string slice using the writer function.
///
/// # Parameters
/// - `src`: The string slice to be written.
/// - `writer`: The function to write the string slice.
///
/// # Returns
/// A `Result` indicating the success or failure of the write operation.
#[inline(always)]
pub(crate) fn write<R>(src: &str, writer: &mut impl Writer<R>) -> Result<(), R> {
    (writer)(src)
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
pub(crate) unsafe fn write_slice<R>(
    start: *const u8,
    end: *const u8,
    writer: &mut impl Writer<R>,
) -> Result<(), R> {
    unsafe {
        write(
            str::from_utf8_unchecked(slice::from_raw_parts(start, end.distance(start))),
            writer,
        )
    }
}

/// A macro for creating a builder function that appends a string to a `String`.
///
/// # Parameters
/// - `$name`: The name of the builder function.
/// - `$fn`: The function to use for the builder.
/// - `$fn_name`: The name of the function to use for the builder.
/// - `$builder`: The type of the builder.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "fmt")]
macro_rules! builder_fmt {
    ($name:ident, $fn:path, $fn_name:ident, $builder:ty) => {
        fn $name(haystack: &str, buffer: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            use $fn;
            let writer = |s: &str| buffer.write_str(s);
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
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "fmt")]
macro_rules! struct_display {
    ($name:ident, $internal:ident, $body:expr, $builder:ty) => {
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
///
/// # Parameters
/// - `$name`: The name of the builder function.
/// - `$fn`: The function to use for the builder.
/// - `$fn_name`: The name of the function to use for the builder.
/// - `$builder`: The type of the builder.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "string")]
macro_rules! builder_string {
    ($name:ident, $fn:path, $fn_name:ident, $builder:ty) => {
        pub fn $name(haystack: &str, buffer: &mut String) {
            use $fn;
            let writer = |s: &str| {
                buffer.push_str(s);
                Ok::<(), ()>(())
            };
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
///
/// # Parameters
/// - `$name`: The name of the builder function.
/// - `$fn`: The function to use for the builder.
/// - `$fn_name`: The name of the function to use for the builder.
/// - `$builder`: The type of the builder.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "bytes")]
macro_rules! builder_bytes {
    ($name:ident, $fn:path, $fn_name:ident, $builder:ty) => {
        pub fn $name(haystack: &str, buffer: &mut Vec<u8>) {
            use $fn;
            let writer = |s: &str| {
                buffer.extend_from_slice(s.as_bytes());
                Ok::<(), ()>(())
            };
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
