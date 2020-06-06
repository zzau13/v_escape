#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_scalar {
    ($($t:tt)+) => {
        #[inline]
        pub fn escape(bytes: &[u8], fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            use std::str::from_utf8_unchecked;
            let mut start = 0;

            for (i, b) in bytes.iter().enumerate() {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == *b {
                            _v_escape_bodies_exact_one!(
                                $byte,
                                $quote,
                                (),
                                i,
                                *b,
                                start,
                                fmt,
                                bytes,
                                _v_escape_escape_body
                            );
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
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
                    };
                }

                _inside!(impl $($t)+);
            }

            fmt.write_str(unsafe { from_utf8_unchecked(&bytes[start..]) })?;

            Ok(())
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_scalar_ptr {
    ($($t:tt)+) => {
        #[inline]
        pub unsafe fn v_escape(bytes: &[u8], buf: &mut [u8]) -> Option<usize> {
            let mut buf_cur = 0;
            let mut start = 0;

            for (i, b) in bytes.iter().enumerate() {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == *b {
                            _v_escape_bodies_exact_one_ptr!(
                                $byte,
                                $quote,
                                (),
                                i,
                                *b,
                                start,
                                buf_cur,
                                buf,
                                bytes,
                                _v_escape_escape_body_ptr
                            );
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        _v_escape_bodies_ptr!(
                            $T,
                            $Q,
                            $Q_LEN,
                            i,
                            *b,
                            start,
                            buf_cur,
                            buf,
                            bytes,
                            _v_escape_escape_body_ptr
                        );
                    };
                }

                _inside!(impl $($t)+);
            }

            let len = bytes.len();
            if start < len {
                _v_escape_write_ptr!(buf_cur, buf, &bytes[start..len], len - start);
            }

            Some(buf_cur)
        }
    };
}
