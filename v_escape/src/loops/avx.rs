#[macro_export]
/// Macro `loop_m256_128` is the main loop that searches in byte slice with a bit mask
/// using `avx2` optimizations.
///
/// ## The following macros must be defined
///
/// * `write_mask(mask: {integer}, ptr: *const u8)` writes operation of full mask.
///
/// * `write_forward(mask: {integer}, until: usize})` writes operation of sliced mask
///
/// * `masking(a: __m256i) -> __m256i` creates a mask from __m256i
///
/// ## Example
///
/// ```
/// #[macro_use]
/// extern crate v_escape;
///
/// #[cfg(all(target_feature = "avx2", target_arch = "x86_64"))]
/// unsafe fn memchr(n1: u8, bytes: &[u8]) -> Option<usize> {
///     use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_set1_epi8};
///
///     let v_a = _mm256_set1_epi8(n1 as i8);
///
///     let len = bytes.len();
///     let start_ptr = bytes.as_ptr();
///     let mut ptr = start_ptr;
///
///     macro_rules! write_mask {
///         ($mask:ident, $ptr:ident) => {{
///             return Some(_v_escape_sub!($ptr, start_ptr) + $mask.trailing_zeros() as usize);
///         }};
///     }
///
///     macro_rules! write_forward {
///         ($mask: ident, $align:ident) => {{
///             let cur = $mask.trailing_zeros() as usize;
///
///                 if cur < $align {
///                    return Some(_v_escape_sub!(ptr, start_ptr) + cur);
///                 }
///         }};
///     }
///
///     macro_rules! masking {
///         ($a:ident) => {{
///             _mm256_cmpeq_epi8($a, v_a)
///         }};
///     }
///
///     loop_m256_128!(len, ptr, start_ptr, bytes);
///
///     None
/// }
///
/// # #[cfg(not(all(target_feature = "avx2", target_arch = "x86_64")))]
/// # fn main() {
/// # }
/// # #[cfg(all(target_feature = "avx2", target_arch = "x86_64"))]
/// # fn main() {
/// assert_eq!(unsafe { memchr(b'a', b"b") }, None);
/// assert_eq!(unsafe { memchr(b'a', b"ba") }, Some(1));
/// # }
/// ```
macro_rules! loop_m256_128 {
    ($len:ident, $ptr:ident, $start_ptr:ident, $bytes:ident) => {{
        #[allow(unused_imports)]
        use std::arch::x86_64::{
            __m256i, _mm256_load_si256, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_or_si256,
        };

        const VECTOR_SIZE: usize = ::std::mem::size_of::<__m256i>();
        const VECTOR_ALIGN: usize = VECTOR_SIZE - 1;
        const LOOP_SIZE: usize = 4 * VECTOR_SIZE;

        // If the string length is less than VECTOR_SIZE
        // unaligned function _mm256_loadu_si256 is used
        if $len < VECTOR_SIZE {
            #[allow(unused_mut)]
            let mut mask = {
                let a = _mm256_loadu_si256($ptr as *const __m256i);
                _mm256_movemask_epi8(masking!(a))
            };

            if mask != 0 {
                write_forward!(mask, $len);
            }
        // If string length is larger than VECTOR_SIZE, then it can be sliced into more than one
        // set of VECTOR_SIZE elements and processed
        } else {
            let end_ptr = $bytes[$len..].as_ptr();

            // Aligning pointer by using `_mm256_loadu_si256` on unaligned bytes.
            {
                let align = VECTOR_SIZE - ($start_ptr as usize & VECTOR_ALIGN);
                if align < VECTOR_SIZE {
                    #[allow(unused_mut)]
                    let mut mask = {
                        let a = _mm256_loadu_si256($ptr as *const __m256i);
                        _mm256_movemask_epi8(masking!(a))
                    };

                    if mask != 0 {
                        write_forward!(mask, align);
                    }
                    // Aligning pointer
                    $ptr = $ptr.add(align);
                }
            }

            debug_assert!($start_ptr <= $ptr && $start_ptr <= end_ptr.sub(VECTOR_SIZE));

            // Process all aligned slices with at least one set of length `LOOP_SIZE`
            if LOOP_SIZE <= $len {
                while $ptr <= end_ptr.sub(LOOP_SIZE) {
                    debug_assert_eq!(0, ($ptr as usize) % VECTOR_SIZE);

                    // Using function `_mm256_load_si256` for faster behavior on aligned bytes.
                    // Getting 4 sets of length `VECTOR_SIZE` each (`LOOP_SIZE=4*VECTOR_SIZE`)
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

                    // Combining the 4 sets using `or` logic operator by pairs. Then we write the
                    // mask for the 4 sets of `VECTOR_SIZE` elements each, and make the pointer
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

            // When the rest of string has a length greater then `VECTOR_SIZE`
            // but less than `LOOP_SIZE`, we process it `VECTOR_SIZE` bits at
            // a time until there are left less then `VECTOR_SIZE` elements
            while $ptr <= end_ptr.sub(VECTOR_SIZE) {
                debug_assert_eq!(0, ($ptr as usize) % VECTOR_SIZE);

                #[allow(unused_mut)]
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

            // At this point at most there is less then `VECTOR_SIZE` elements
            // so the macro `write_forward` is used to finalize de process
            if $ptr < end_ptr {
                debug_assert_eq!(0, ($ptr as usize) % VECTOR_SIZE);

                #[allow(unused_mut)]
                let mut mask = {
                    let a = _mm256_load_si256($ptr as *const __m256i);
                    _mm256_movemask_epi8(masking!(a))
                };
                let end = _v_escape_sub!(end_ptr, $ptr);

                if mask != 0 {
                    write_forward!(mask, end);
                }
            }
        }
    }};
}
