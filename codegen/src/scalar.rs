#[macro_export]
#[doc(hidden)]
macro_rules! escape_scalar {
    ($($t:tt)+) => {
        #[inline]
        pub fn escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
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
                            $crate::bodies_exact_one!(
                                $byte,
                                $quote,
                                (),
                                sub(ptr, start_ptr),
                                *ptr,
                                start,
                                fmt,
                                bytes,
                                $crate::escape_body
                            );
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        $crate::bodies!(
                            $T,
                            $Q,
                            $Q_LEN,
                            sub(ptr, start_ptr),
                            *ptr,
                            start,
                            fmt,
                            bytes,
                            $crate::escape_body
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
macro_rules! escape_scalar_bytes {
    ($($t:tt)+) => {
        #[inline]
        pub unsafe fn b_escape<B: $crate::Buffer>(bytes: &[u8], buf: &mut B) {
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();

            let mut ptr = start_ptr;

            let mut start = 0;

            while ptr < end_ptr {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == *ptr {
                            $crate::bodies_exact_one_bytes!(
                                $byte,
                                $quote,
                                (),
                                sub(ptr, start_ptr),
                                *ptr,
                                start,
                                bytes,
                                buf,
                                $crate::escape_body_bytes
                            );
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        $crate::bodies_bytes!(
                            $T,
                            $Q,
                            $Q_LEN,
                            sub(ptr, start_ptr),
                            *ptr,
                            start,
                            bytes,
                            buf,
                            $crate::escape_body_bytes
                        );
                    };
                }

                _inside!(impl $($t)+);

                ptr = ptr.offset(1);
            }

            // Write since start to the end of the slice
            debug_assert!(start <= len);
            if start < len {
                $crate::write_bytes!(&bytes[start..], buf);
            }
        }
    };
}
