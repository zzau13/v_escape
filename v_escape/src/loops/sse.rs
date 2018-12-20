/// Main loop for search in byte slice with bit mask
///
/// #### The following macros must be defined:
/// * `write_mask(mask: {integer})` do operation at mask
///
/// * `masking(a: __m128i, len: {integer}) -> {integer}` make a mask from __m128i
///
/// #### Example
///
/// ```
///
/// #[macro_use]
/// extern crate v_escape;
///
/// #[cfg(all(target_feature = "sse4.2", target_arch = "x86_64"))]
/// unsafe fn memchr4(n0: u8, n1: u8, n2: u8, n3: u8, bytes: &[u8]) -> Option<usize> {
///     use std::arch::x86_64::{_mm_cmpestri, _mm_setr_epi8};
///     const NEEDLE_LEN: i32 = 4;
///     let needle = _mm_setr_epi8(
///         n0 as i8, n1 as i8, n2 as i8, n3 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
///     );
///
///     let len = bytes.len();
///     let start_ptr = bytes.as_ptr();
///     let mut ptr = start_ptr;
///
///     macro_rules! write_mask {
///         ($mask:ident) => {{
///             if $mask < 16 {
///                 return Some(_v_escape_sub!(ptr, start_ptr) + $mask as usize);
///             }
///         }};
///     }
///
///     macro_rules! masking {
///         ($a:ident, $len:ident) => {{
///             _mm_cmpestri(needle, NEEDLE_LEN, $a, $len as i32, 0)
///         }};
///     }
///
///     loop_m128!(len, ptr, start_ptr, bytes);
///
///     None
/// }
///
/// # #[cfg(not(all(target_feature = "sse4.2", target_arch = "x86_64")))]
/// # fn main() {
/// # }
/// # #[cfg(all(target_feature = "sse4.2", target_arch = "x86_64"))]
/// # fn main() {
/// assert_eq!(unsafe { memchr4(b'a', b'b', b'c', b'd', b"e") }, None);
/// assert_eq!(unsafe { memchr4(b'a', b'b', b'c', b'd', b"abcd") }, Some(0));
/// # }
/// ```
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
