#[macro_export]
macro_rules!  _v_escape_escape_sse {
    (($T:ident, $Q:ident, $needle_len:ident) $($needle:tt, )+) => {
        #[inline]
        #[target_feature(enable = "sse4.2")]
        pub unsafe fn escape(bytes: &[u8], fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            #[allow(unused_imports)]
            use std::arch::x86_64::{_mm_cmpestrm, _mm_extract_epi16, _mm_setr_epi8};
            const NEEDLE_LEN: i32 = $needle_len as i32;
            let needle = _mm_setr_epi8($($needle as i8,)+);

            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let mut ptr = bytes.as_ptr();
            let mut start = 0;

            // Macro to write with mask
            macro_rules! write_mask {
                ($mask:ident) => {{
                    if $mask != 0 {
                        // Reference to the start of mask
                        let at = _v_escape_sub!(ptr, start_ptr);
                        // Get to the first possible escape character avoiding zeros
                        let mut cur = $mask.trailing_zeros() as usize;

                        loop {
                            // Writing in `$fmt` with `$mask`
                            // The main loop will break when mask == 0
                            debug_assert_ne!($T[*ptr.add(cur) as usize], NEEDLE_LEN as u8);
                            _v_escape_mask_body!(at + cur, start, fmt, bytes, $Q[$T[*ptr.add(cur) as usize] as usize]);

                            // Create binary vector of all zeros except
                            // position `$curr` and xor operation with `$mask`
                            $mask ^= 1 << cur;
                            // Test vs Check  if `$mask` is empty
                            if $mask == 0 {
                                break;
                            }

                            // Get to the next possible escape character avoiding zeros
                            cur = $mask.trailing_zeros() as usize;
                        }

                        debug_assert_eq!(at, _v_escape_sub!(ptr, start_ptr))
                    }
                }};
            }

            macro_rules! masking {
                ($a:ident, $len:ident) => {
                    _mm_extract_epi16(_mm_cmpestrm(needle, NEEDLE_LEN, $a, $len as i32, 0), 0) as i16
                };
            }

            loop_m128!(len, ptr, start_ptr, bytes);

            // Write since start to the end of the slice
            debug_assert!(start <= len);
            if start < len {
                fmt.write_str(::std::str::from_utf8_unchecked(&bytes[start..len]))?;
            }

            Ok(())
        }
    };
}

#[macro_export]
macro_rules!  _v_escape_sized_sse {
    (($S:ident, $needle_len:ident) $($needle:tt, )+) => {
        #[inline]
        #[target_feature(enable = "sse4.2")]
        pub unsafe fn size(bytes: &[u8]) -> usize {
            #[allow(unused_imports)]
            use std::arch::x86_64::{_mm_cmpestrm, _mm_extract_epi16, _mm_setr_epi8};
            const NEEDLE_LEN: i32 = $needle_len as i32;
            let needle = _mm_setr_epi8($($needle as i8,)+);

            let start_ptr = bytes.as_ptr();
            let mut ptr = bytes.as_ptr();
            let len = bytes.len();
            let mut acc = bytes.len();

            // Macro to write with mask
            macro_rules! write_mask {
                ($mask:ident) => {{
                    if $mask != 0 {
                        // Get to the first possible escape character avoiding zeros
                        let mut cur = $mask.trailing_zeros() as usize;

                        loop {
                            // The main loop will break when mask == 0
                             _v_escape_size_bodies!($S, acc, *ptr.add(cur));

                            // Create binary vector of all zeros except
                            // position `$curr` and xor operation with `$mask`
                            $mask ^= 1 << cur;
                            // Test vs Check  if `$mask` is empty
                            if $mask == 0 {
                                break;
                            }

                            // Get to the next possible escape character avoiding zeros
                            cur = $mask.trailing_zeros() as usize;
                        }
                    }
                }};
            }

            macro_rules! masking {
                ($a:ident, $len:ident) => {
                    _mm_extract_epi16(_mm_cmpestrm(needle, NEEDLE_LEN, $a, $len as i32, 0), 0) as i16
                };
            }

            loop_m128!(len, ptr, start_ptr, bytes);

            acc
        }
    };
}
