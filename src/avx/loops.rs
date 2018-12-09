/// Main loop for search in byte slice with bit mask
///
/// The following macros must be defined:
/// - write_mask(mask: {integer}, ptr: *const u8).
/// do op at full mask
/// - write_forward(mask: {integer}, until: usize})
/// do op at sliced mask
/// - masking(a: __m256i) -> __m256i masking at i8
/// make a mask from __m256i
// TODO: document in detail
// TODO: simple unit test
#[macro_export]
macro_rules! loops {
    ($len:ident, $ptr:ident, $start_ptr:ident, $bytes:ident) => {{
        const VECTOR_SIZE: usize = size_of::<__m256i>();
        const VECTOR_ALIGN: usize = VECTOR_SIZE - 1;
        const LOOP_SIZE: usize = 4 * VECTOR_SIZE;

        if $len < VECTOR_SIZE {
            let mut mask = {
                let a = _mm256_lddqu_si256($ptr as *const __m256i);
                _mm256_movemask_epi8(masking!(a))
            };

            if mask != 0 {
                write_forward!(mask, $len);
            }
        } else {
            let end_ptr = $bytes[$len..].as_ptr();

            {
                let align = VECTOR_SIZE - ($start_ptr as usize & VECTOR_ALIGN);
                if align < VECTOR_SIZE {
                    let mut mask = {
                        let a = _mm256_loadu_si256($ptr as *const __m256i);
                        _mm256_movemask_epi8(masking!(a))
                    };

                    if mask != 0 {
                        write_forward!(mask, align);
                    }
                    $ptr = $ptr.add(align);
                }
            }

            debug_assert!($start_ptr <= $ptr && $start_ptr <= end_ptr.sub(VECTOR_SIZE));

            if LOOP_SIZE <= $len {
                while $ptr <= end_ptr.sub(LOOP_SIZE) {
                    debug_assert_eq!(0, ($ptr as usize) % VECTOR_SIZE);
                    let cmp_a = {
                        let a = _mm256_load_si256($ptr as *const __m256i);
                        masking!(a)
                    };

                    let cmp_b = {
                        let a = _mm256_load_si256($ptr.add(VECTOR_SIZE) as *const __m256i);
                        masking!(a)
                    };

                    let cmp_c = {
                        let a = _mm256_load_si256($ptr.add(VECTOR_SIZE * 2) as *const __m256i);
                        masking!(a)
                    };

                    let cmp_d = {
                        let a = _mm256_load_si256($ptr.add(VECTOR_SIZE * 3) as *const __m256i);
                        masking!(a)
                    };

                    if _mm256_movemask_epi8(_mm256_or_si256(_mm256_or_si256(cmp_a, cmp_b), _mm256_or_si256(cmp_c, cmp_d))) != 0 {
                        let mut mask = _mm256_movemask_epi8(cmp_a);
                        if mask != 0 {
                            write_mask!(mask, $ptr);
                        }

                        mask = _mm256_movemask_epi8(cmp_b);
                        if mask != 0 {
                            let ptr = $ptr.add(VECTOR_SIZE);
                            write_mask!(mask, ptr);
                        }

                        mask = _mm256_movemask_epi8(cmp_c);
                        if mask != 0 {
                            let ptr = $ptr.add(VECTOR_SIZE * 2);
                            write_mask!(mask, ptr);
                        }

                        mask = _mm256_movemask_epi8(cmp_d);
                        if mask != 0 {
                            let ptr = $ptr.add(VECTOR_SIZE * 3);
                            write_mask!(mask, ptr);
                        }
                    }

                    $ptr = $ptr.add(LOOP_SIZE);
                }
            }

            while $ptr <= end_ptr.sub(VECTOR_SIZE) {
                debug_assert_eq!(0, ($ptr as usize) % VECTOR_SIZE);

                let mut mask = {
                    let a = _mm256_load_si256($ptr as *const __m256i);
                    _mm256_movemask_epi8(masking!(a))
                };

                if mask != 0 {
                    write_mask!(mask, $ptr);
                }
                $ptr = $ptr.add(VECTOR_SIZE);
            }

            debug_assert!(end_ptr.sub(VECTOR_SIZE) < $ptr);

            if $ptr < end_ptr {
                debug_assert_eq!(0, ($ptr as usize) % VECTOR_SIZE);

                let mut mask = {
                    let a = _mm256_load_si256($ptr as *const __m256i);
                    _mm256_movemask_epi8(masking!(a))
                };
                let end = sub(end_ptr, $ptr);

                if mask != 0 {
                    write_forward!(mask, end);
                }
            }
        }
    }};
}
