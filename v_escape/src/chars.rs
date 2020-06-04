#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_char {
    ($($t:tt)+) => {
        #[inline]
        pub fn escape_char(c: char, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {

            if c.is_ascii() {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == c as u8 {
                            return fmt.write_str($quote)
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        let c = $T[c as usize] as usize;
                        if c < $Q_LEN {
                          return fmt.write_str($Q[c]);
                        }
                    };
                }

                _inside!(impl $($t)+);
            }

            use std::fmt::Write;
            fmt.write_char(c)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_char_ptr {
    ($($t:tt)+) => {
        #[inline]
        pub unsafe fn v_escape_char(c: char, buf: &mut [u8]) -> Option<usize> {
            let max = buf.len();
            let buf_ptr = buf.as_mut_ptr();

            if c.is_ascii() {
                macro_rules! _inside {
                    (impl one $byte:ident, $quote:ident) => {
                        if $byte == c as u8 {
                            let mut buf_cur = 0;
                            _v_escape_write_ptr!(buf_cur, buf_ptr, $quote.as_bytes(), $quote.len(), max);
                            return Some(buf_cur);
                        }
                    };
                    (impl $T:ident, $Q:ident, $Q_LEN:ident) => {
                        let c = $T[c as usize] as usize;
                        if c < $Q_LEN {
                            let mut buf_cur = 0;
                            let quote = $Q[c];
                            _v_escape_write_ptr!(buf_cur, buf_ptr, quote.as_bytes(), quote.len(), max);
                            return Some(buf_cur);
                        }
                    };
                }

                _inside!(impl $($t)+);
                // Ascii length is one byte
                if 0 < max {
                    buf_ptr.write(c as u8);
                    return Some(1);
                } else {
                    return None;
                }
            }


            // https://doc.rust-lang.org/std/primitive.char.html#panics-2
            if 3 < max {
                Some(c.encode_utf8(buf).len())
            } else {
                None
            }
        }
    };
}
