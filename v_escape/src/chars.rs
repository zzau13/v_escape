#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_char {
    ($($t:tt)+) => {
        #[inline]
        pub fn escape_char(c: char, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {

            if c.len_utf8() == 1 {
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
