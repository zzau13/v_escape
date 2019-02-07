#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_sse {
    (($T:ident, $Q:ident, $needle_len:ident) $($needle:tt, )+) => {
        #[inline]
        #[target_feature(enable = "sse4.2")]
        pub unsafe fn escape(bytes: &[u8], fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            use std::arch::x86_64::{_mm_cmpestrm, _mm_extract_epi16, _mm_setr_epi8};

            const NEEDLE_LEN: i32 = $needle_len as i32;

            let len = bytes.len();

            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = bytes.as_ptr();

            let mut start = 0;


            // Macro to write with mask
            macro_rules! write_mask {
                ($mask:ident) => {{
                    if $mask != 0 {
                        // Reference to the start of mask
                        let at = _v_escape_sub!(ptr, start_ptr);
                        // Get to the first escape character avoiding zeros
                        let mut cur = $mask.trailing_zeros() as usize;

                        loop {
                            // Writing in `$fmt` with `$mask`
                            // The main loop will break when mask == 0
                            _v_escape_bodies_exact!($T, $Q, NEEDLE_LEN, at + cur, *ptr.add(cur), start, fmt, bytes, _v_escape_mask_body);

                            // Create binary vector of all zeros except
                            // position `$curr` and xor operation with `$mask`
                            $mask ^= 1 << cur;
                            // Check if `$mask` is empty
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

            use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_loadu_si128, _mm_movemask_epi8};

            const M128_VECTOR_SIZE: usize = ::std::mem::size_of::<__m128i>();
            const M128_VECTOR_ALIGN: usize = M128_VECTOR_SIZE - 1;

            if len < M128_VECTOR_SIZE {
                 while ptr < end_ptr {
                     _v_escape_bodies!(
                        $T,
                        $Q,
                        $needle_len,
                        _v_escape_sub!(ptr, start_ptr),
                        *ptr,
                        start,
                        fmt,
                        bytes,
                        _v_escape_mask_body
                    );
                    ptr = ptr.offset(1);
                 }
            } else {
                let needle = _mm_setr_epi8($($needle as i8,)+);

                macro_rules! masking {
                    ($a:ident, $len:ident) => {
                        _mm_extract_epi16(_mm_cmpestrm(needle, NEEDLE_LEN, $a, $len as i32, 0), 0) as i16
                    };
                }

                // Write mask for unaligned elements from the start
                // of the vector and aligning pointer
                {
                    // Calculating index of aligned pointer
                    let align = M128_VECTOR_SIZE - (start_ptr as usize & M128_VECTOR_ALIGN);
                    if align < M128_VECTOR_SIZE {
                        let mut mask = {
                            let a = _mm_loadu_si128(ptr as *const __m128i);
                            masking!(a, align)
                        };
                        // Writing mask for unaligned elements
                        write_mask!(mask);
                        // Aligning pointer
                        ptr = ptr.add(align);
                    }
                }
                // Process all aligned slices with at least one set of length `M128_VECTOR_SIZE`
                while ptr <= end_ptr.sub(M128_VECTOR_SIZE) {
                    debug_assert_eq!(0, (ptr as usize) % M128_VECTOR_SIZE);

                    let mut mask = {
                        let a = _mm_load_si128(ptr as *const __m128i);
                        masking!(a, M128_VECTOR_SIZE)
                    };

                    write_mask!(mask);
                    ptr = ptr.add(M128_VECTOR_SIZE);
                }

                debug_assert!(end_ptr.sub(M128_VECTOR_SIZE) < ptr);

                // At this point at most there is less than `M128_VECTOR_SIZE` elements
                // so the macro `write_mask` is used to the last elements
                if ptr < end_ptr {
                    debug_assert_eq!(0, (ptr as usize) % M128_VECTOR_SIZE);

                    let end = _v_escape_sub!(end_ptr, ptr);
                    let mut mask = {
                        let a = _mm_load_si128(ptr as *const __m128i);
                        masking!(a, end)
                    };

                    write_mask!(mask);
                }
            }
            // Write since start to the end of the slice
            debug_assert!(start <= len);
            if start < len {
                fmt.write_str(::std::str::from_utf8_unchecked(&bytes[start..len]))?;
            }

            Ok(())
        }
    };
}
