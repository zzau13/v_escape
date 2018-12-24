#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_scalar {
    ($T:ident, $Q:ident, $Q_LEN:ident) => {
        #[inline]
        pub fn escape(bytes: &[u8], fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            use std::str::from_utf8_unchecked;
            let mut start = 0;

            for (i, b) in bytes.iter().enumerate() {
                _v_escape_bodies!(
                    $T,
                    $Q,
                    $Q_LEN,
                    i,
                    *b,
                    start,
                    fmt,
                    bytes,
                    _v_escape_escape_body
                );
            }

            fmt.write_str(unsafe { from_utf8_unchecked(&bytes[start..]) })?;

            Ok(())
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_sized_scalar {
    ($S:ident) => {
        #[inline]
        pub fn size(bytes: &[u8]) -> usize {
            let mut acc = bytes.len();

            for b in bytes {
                _v_escape_size_bodies!($S, acc, *b);
            }

            acc
        }
    };
}
