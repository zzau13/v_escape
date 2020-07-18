#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_scalar {
    ($($t:tt)+) => {
        #[inline]
        pub fn escape(bytes: &[u8], fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            use std::str::from_utf8_unchecked;

            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();

            let mut ptr = start_ptr;

            let start_ptr = bytes.as_ptr();
            let mut start = 0;

            unsafe {

            while ptr < end_ptr {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == *ptr {
                            _v_escape_bodies_exact_one!(
                                $byte,
                                $quote,
                                (),
                                _v_escape_sub!(ptr, start_ptr),
                                *ptr,
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
                            _v_escape_sub!(ptr, start_ptr),
                            *ptr,
                            start,
                            fmt,
                            bytes,
                            _v_escape_escape_body
                        );
                    };
                }

                _inside!(impl $($t)+);

                ptr = ptr.offset(1);
            }

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
        pub unsafe fn f_escape(bytes: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();

            let mut ptr = start_ptr;

            let mut buf_cur = 0;
            let mut start = 0;

            while ptr < end_ptr {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == *ptr {
                            _v_escape_bodies_exact_one_ptr!(
                                $byte,
                                $quote,
                                (),
                                _v_escape_sub!(ptr, start_ptr),
                                *ptr,
                                start,
                                buf_cur,
                                buf,
                                start_ptr,
                                _v_escape_escape_body_ptr
                            );
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        _v_escape_bodies_ptr!(
                            $T,
                            $Q,
                            $Q_LEN,
                            _v_escape_sub!(ptr, start_ptr),
                            *ptr,
                            start,
                            buf_cur,
                            buf,
                            start_ptr,
                            _v_escape_escape_body_ptr
                        );
                    };
                }

                _inside!(impl $($t)+);

                ptr = ptr.offset(1);
            }

            // Write since start to the end of the slice
            debug_assert!(start <= len);
            if start < len {
                let len = len - start;
                _v_escape_write_ptr!(buf_cur, buf, start_ptr.add(start), len);
            }

            Some(buf_cur)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_scalar_bytes {
    ($($t:tt)+) => {
        #[inline]
        pub unsafe fn b_escape<B: v_escape::Buffer>(bytes: &[u8], buf: &mut B) {
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();

            let mut ptr = start_ptr;

            let mut start = 0;

            while ptr < end_ptr {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == *ptr {
                            _v_escape_bodies_exact_one_bytes!(
                                $byte,
                                $quote,
                                (),
                                _v_escape_sub!(ptr, start_ptr),
                                *ptr,
                                start,
                                bytes,
                                buf,
                                _v_escape_escape_body_bytes
                            );
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        _v_escape_bodies_bytes!(
                            $T,
                            $Q,
                            $Q_LEN,
                            _v_escape_sub!(ptr, start_ptr),
                            *ptr,
                            start,
                            bytes,
                            buf,
                            _v_escape_escape_body_bytes
                        );
                    };
                }

                _inside!(impl $($t)+);

                ptr = ptr.offset(1);
            }

            // Write since start to the end of the slice
            debug_assert!(start <= bytes.len());
            if start < bytes.len() {
                _v_escape_write_bytes!(&bytes[start..], buf);
            }
        }
    };
}
