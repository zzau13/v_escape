/// Main loop for search in byte slice with bit mask
///
/// The following macros must be defined:
/// - write_mask(mask: {integer}).
/// do op at mask
/// - masking(a: __m128i, len: {integer}) -> {integer}
/// make a mask from __m128i
// TODO: document in detail
#[macro_export]
macro_rules! loop_m128 {
    ($len:ident, $ptr:ident, $start_ptr:ident, $bytes:ident) => {{
        #[allow(unused_imports)]
        use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_loadu_si128};

        const VECTOR_SIZE: usize = ::std::mem::size_of::<__m128i>();
        const VECTOR_ALIGN: usize = VECTOR_SIZE - 1;

        if $len < VECTOR_SIZE {
            #[allow(unused_mut)]
            let mut mask = {
                let a = _mm_loadu_si128($ptr as *const __m128i);
                masking!(a, $len)
            };

            write_mask!(mask);
        } else {
            let end_ptr = $bytes[$len..].as_ptr();

            {
                let align = VECTOR_SIZE - ($start_ptr as usize & VECTOR_ALIGN);
                if align < VECTOR_SIZE {
                    #[allow(unused_mut)]
                    let mut mask = {
                        let a = _mm_loadu_si128($ptr as *const __m128i);
                        masking!(a, align)
                    };

                    write_mask!(mask);
                    $ptr = $ptr.add(align);
                }
            }

            while $ptr <= end_ptr.sub(VECTOR_SIZE) {
                debug_assert_eq!(0, ($ptr as usize) % VECTOR_SIZE);

                #[allow(unused_mut)]
                let mut mask = {
                    let a = _mm_load_si128($ptr as *const __m128i);
                    masking!(a, VECTOR_SIZE)
                };

                write_mask!(mask);
                $ptr = $ptr.add(VECTOR_SIZE);
            }

            debug_assert!(end_ptr.sub(VECTOR_SIZE) < $ptr);

            if $ptr < end_ptr {
                debug_assert_eq!(0, ($ptr as usize) % VECTOR_SIZE);

                let end = _v_escape_sub!(end_ptr, $ptr);
                #[allow(unused_mut)]
                let mut mask = {
                    let a = _mm_load_si128($ptr as *const __m128i);
                    masking!(a, end)
                };

                write_mask!(mask);
            }
        }
    }};
}
