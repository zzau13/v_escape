#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_loop_avx2  {
    (($len:ident, $ptr:ident, $start_ptr:ident, $end_ptr:ident, $start:ident, $fmt:ident, $bytes:ident)
    ($T:ident, $Q:ident, $Q_LEN:ident) $($t:tt ,)+) => {
        use std::arch::x86_64::{
            __m256i, _mm256_load_si256, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_or_si256,
        };

        const M256_VECTOR_SIZE: usize = ::std::mem::size_of::<__m256i>();
        const M256_VECTOR_ALIGN: usize = M256_VECTOR_SIZE - 1;
        const LOOP_SIZE: usize = 4 * M256_VECTOR_SIZE;

        if $len < M256_VECTOR_SIZE {
            _v_escape_loop_sse2!(
                ($len, $ptr, $start_ptr, $end_ptr, $start, $fmt, $bytes)
                ($T, $Q, $Q_LEN) $($t ,)+
            );
        } else {
            _v_escape_translations!($($t, )+);

            // Aligning pointer by using `_mm256_loadu_si256` on unaligned bytes.
            {
                let align = M256_VECTOR_SIZE - ($start_ptr as usize & M256_VECTOR_ALIGN);
                if align < M256_VECTOR_SIZE {
                    let mut mask = {
                        let a = _mm256_loadu_si256($ptr as *const __m256i);
                        _mm256_movemask_epi8(masking!(a))
                    };

                    write_forward!(mask, align);
                    // Aligning pointer
                    $ptr = $ptr.add(align);
                }
            }

            debug_assert!($start_ptr <= $ptr && $start_ptr <= $end_ptr.sub(M256_VECTOR_SIZE));

            // Process all aligned slices with at least one set of $length `LOOP_SIZE`
            if LOOP_SIZE <= $len {
                while $ptr <= $end_ptr.sub(LOOP_SIZE) {
                    debug_assert_eq!(0, ($ptr as usize) % M256_VECTOR_SIZE);

                    // Using function `_mm256_load_si256` for faster behavior on aligned bytes.
                    // Getting 4 sets of $length `M256_VECTOR_SIZE` each (`LOOP_SIZE=4*M256_VECTOR_SIZE`)
                    let cmp_a = {
                        let a = _mm256_load_si256($ptr as *const __m256i);
                        masking!(a)
                    };

                    let cmp_b = {
                        let a = _mm256_load_si256($ptr.add(M256_VECTOR_SIZE) as *const __m256i);
                        masking!(a)
                    };

                    let cmp_c = {
                        let a = _mm256_load_si256($ptr.add(M256_VECTOR_SIZE * 2) as *const __m256i);
                        masking!(a)
                    };

                    let cmp_d = {
                        let a = _mm256_load_si256($ptr.add(M256_VECTOR_SIZE * 3) as *const __m256i);
                        masking!(a)
                    };

                    // Combining the 4 sets using `or` logic operator by pairs. Then we write the
                    // mask for the 4 sets of `M256_VECTOR_SIZE` elements each, and make the pointer
                    // point to the next `LOOP_SIZE` elements
                    if _mm256_movemask_epi8(_mm256_or_si256(
                        _mm256_or_si256(cmp_a, cmp_b),
                        _mm256_or_si256(cmp_c, cmp_d),
                    )) != 0
                    {
                        let mut mask = _mm256_movemask_epi8(cmp_a);
                        if mask != 0 {
                            write_mask!(mask, $ptr);
                        }

                        mask = _mm256_movemask_epi8(cmp_b);
                        if mask != 0 {
                            let $ptr = $ptr.add(M256_VECTOR_SIZE);
                            write_mask!(mask, $ptr);
                        }

                        mask = _mm256_movemask_epi8(cmp_c);
                        if mask != 0 {
                            let $ptr = $ptr.add(M256_VECTOR_SIZE * 2);
                            write_mask!(mask, $ptr);
                        }

                        mask = _mm256_movemask_epi8(cmp_d);
                        if mask != 0 {
                            let $ptr = $ptr.add(M256_VECTOR_SIZE * 3);
                            write_mask!(mask, $ptr);
                        }
                    }

                    $ptr = $ptr.add(LOOP_SIZE);
                }
            }

            // When the rest of string has a $length greater then `M256_VECTOR_SIZE`
            // but less than `LOOP_SIZE`, we process it `M256_VECTOR_SIZE` bits at
            // a time until there are left less then `M256_VECTOR_SIZE` elements
            while $ptr <= $end_ptr.sub(M256_VECTOR_SIZE) {
                debug_assert_eq!(0, ($ptr as usize) % M256_VECTOR_SIZE);

                let mut mask = {
                    let a = _mm256_load_si256($ptr as *const __m256i);
                    _mm256_movemask_epi8(masking!(a))
                };

                if mask != 0 {
                    write_mask!(mask, $ptr);
                }
                $ptr = $ptr.add(M256_VECTOR_SIZE);
            }

            debug_assert!($end_ptr.sub(M256_VECTOR_SIZE) < $ptr);

            // At this point at most there is less then `M256_VECTOR_SIZE` elements
            // so the macro `write_forward` is used to finalize de process
            if $ptr < $end_ptr {
                debug_assert_eq!(0, ($ptr as usize) % M256_VECTOR_SIZE);

                let mut mask = {
                    let a = _mm256_load_si256($ptr as *const __m256i);
                    _mm256_movemask_epi8(masking!(a))
                };

                if mask != 0 {
                    let end = _v_escape_sub!($end_ptr, $ptr);
                    write_forward!(mask, end);
                }
            }
        }
    };
}
