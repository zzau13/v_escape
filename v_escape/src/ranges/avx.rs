#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_loop_avx2  {
    (($len:ident, $ptr:ident, $start_ptr:ident, $end_ptr:ident, $start:ident, $fmt:ident, $bytes:ident)
    ($T:ident, $Q:ident, $Q_LEN:ident) $($t:tt, )+) => {
        use std::arch::x86_64::{
            __m256i, _mm256_load_si256, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_or_si256,
        };

        const M256_VECTOR_SIZE: usize = ::std::mem::size_of::<__m256i>();

        if $len < M256_VECTOR_SIZE {
            _v_escape_loop_sse2!(
                ($len, $ptr, $start_ptr, $end_ptr, $start, $fmt, $bytes)
                ($T, $Q, $Q_LEN) $($t ,)+
            );
        } else {
            _v_escape_translations!($($t, )+);

            // Aligning pointer by using `_mm256_loadu_si256` on unaligned bytes.
            {
                const M256_VECTOR_ALIGN: usize = M256_VECTOR_SIZE - 1;
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

            _v_escape_avx_main_loop!(($len, $ptr, $end_ptr) $($t, )+);

            debug_assert!($start_ptr <= $ptr && $start_ptr <= $end_ptr.sub(M256_VECTOR_SIZE));

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
