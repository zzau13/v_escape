#[macro_export]
/// Macro `loop_m128` is the main loop that searches in byte slice with a bit mask
/// using `sse4.2` optimizations.
///
/// ## The following macros must be defined:
///
/// * `write_mask(mask: {integer})` do operation at mask
///
/// * `masking(a: __m128i, len: {integer}) -> {integer}` make a mask from __m128i
///
/// ## Example
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
macro_rules! loop_m128 {
    ($len:ident, $ptr:ident, $start_ptr:ident, $bytes:ident) => {{
        #[allow(unused_imports)]
        use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_loadu_si128};

        const VECTOR_SIZE: usize = ::std::mem::size_of::<__m128i>();
        const VECTOR_ALIGN: usize = VECTOR_SIZE - 1;

        if $len < VECTOR_SIZE {
            // If the string length is less than VECTOR_SIZE
            // unaligned function _mm_loadu_si128 is used
            #[allow(unused_mut)]
            let mut mask = {
                let a = _mm_loadu_si128($ptr as *const __m128i);
                masking!(a, $len)
            };

            write_mask!(mask);
        } else {
            // If string length is larger than VECTOR_SIZE,
            // then it can be sliced into more than one set
            // of VECTOR_SIZE elements and processed
            let end_ptr = $bytes[$len..].as_ptr();

            // Write mask for unaligned elements from the start
            // of the vector and aligning pointer
            {
                // Calculating index of aligned pointer
                let align = VECTOR_SIZE - ($start_ptr as usize & VECTOR_ALIGN);
                if align < VECTOR_SIZE {
                    #[allow(unused_mut)]
                    let mut mask = {
                        let a = _mm_loadu_si128($ptr as *const __m128i);
                        masking!(a, align)
                    };
                    // Writing mask for unaligned elements
                    write_mask!(mask);
                    // Aligning pointer
                    $ptr = $ptr.add(align);
                }
            }
            // Process all aligned slices with at least one set of length `VECTOR_SIZE`
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

            // At this point at most there is less than `VECTOR_SIZE` elements
            // so the macro `write_mask` is used to the last elements
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
