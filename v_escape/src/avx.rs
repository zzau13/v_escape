#[macro_export]
#[doc(hidden)]
/// Generate translations
///
/// Defining character interval from ASCII table to create bit masks from slice to be escaped
/// overflow above in addition
macro_rules! _v_escape_translations {
    ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_or_si256,
            _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const B: i8 = $fb;
        const C: i8 = $fc;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_b = _mm256_set1_epi8(B);
        let v_c = _mm256_set1_epi8(C);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_or_si256(_mm256_cmpeq_epi8($a, v_b), _mm256_cmpeq_epi8($a, v_c)),
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                )
            }};
        }
    };
    ($fa:expr, $fb:expr, $fc:expr, 128, ) => {
        use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_or_si256, _mm256_set1_epi8};
        const A: i8 = $fa;
        const B: i8 = $fb;
        const C: i8 = $fc;

        let v_a = _mm256_set1_epi8(A);
        let v_b = _mm256_set1_epi8(B);
        let v_c = _mm256_set1_epi8(C);

        macro_rules! masking {
            ($a:ident) => {{
                _mm256_or_si256(
                    _mm256_or_si256(_mm256_cmpeq_epi8($a, v_a), _mm256_cmpeq_epi8($a, v_b)),
                    _mm256_cmpeq_epi8($a, v_c),
                )
            }};
        }
    };
    ($fa:expr, $fb:expr, 128, ) => {
        use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_or_si256, _mm256_set1_epi8};
        const A: i8 = $fa;
        const B: i8 = $fb;

        let v_a = _mm256_set1_epi8(A);
        let v_b = _mm256_set1_epi8(B);

        macro_rules! masking {
            ($a:ident) => {{
                _mm256_or_si256(_mm256_cmpeq_epi8($a, v_a), _mm256_cmpeq_epi8($a, v_b))
            }};
        }
    };
    ($fa:expr, 128, ) => {
        use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_set1_epi8};
        const A: i8 = $fa;

        let v_a = _mm256_set1_epi8(A);

        macro_rules! masking {
            ($a:ident) => {{
                _mm256_cmpeq_epi8($a, v_a)
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpgt_epi8, _mm256_or_si256, _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;
        const TRANSLATION_C: i8 = ::std::i8::MAX - $rc;
        const BELOW_C: i8 = ::std::i8::MAX - ($rc - $lc) - 1;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm256_set1_epi8(BELOW_B);
        let v_translation_c = _mm256_set1_epi8(TRANSLATION_C);
        let v_below_c = _mm256_set1_epi8(BELOW_C);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_or_si256(
                        _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                        _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_b), v_below_b),
                    ),
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_c), v_below_c),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_or_si256,
            _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;
        const C: i8 = $c;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm256_set1_epi8(BELOW_B);
        let v_c = _mm256_set1_epi8(C);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_or_si256(
                        _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                        _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_b), v_below_b),
                    ),
                    _mm256_cmpeq_epi8($a, v_c),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpgt_epi8, _mm256_or_si256, _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm256_set1_epi8(BELOW_B);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_b), v_below_b),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $b:expr, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_or_si256,
            _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const B: i8 = $b;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_b = _mm256_set1_epi8(B);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                    _mm256_cmpeq_epi8($a, v_b),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, ) => {
        use std::arch::x86_64::{_mm256_add_epi8, _mm256_cmpgt_epi8, _mm256_set1_epi8};
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a)
            }};
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Generate translations
///
/// Defining character interval from ASCII table to create bit masks from slice to be escaped
/// overflow above in addition
macro_rules! _v_escape_translations_128 {
    ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
        use std::arch::x86_64::{
            _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const B: i8 = $fb;
        const C: i8 = $fc;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_b = _mm_set1_epi8(B);
        let v_c = _mm_set1_epi8(C);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_or_si128(_mm_cmpeq_epi8($a, v_b), _mm_cmpeq_epi8($a, v_c)),
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                )
            }};
        }
    };
    ($fa:expr, $fb:expr, $fc:expr, 128, ) => {
        use std::arch::x86_64::{_mm_cmpeq_epi8, _mm_or_si128, _mm_set1_epi8};
        const A: i8 = $fa;
        const B: i8 = $fb;
        const C: i8 = $fc;

        let v_a = _mm_set1_epi8(A);
        let v_b = _mm_set1_epi8(B);
        let v_c = _mm_set1_epi8(C);

        macro_rules! masking_128 {
            ($a:ident) => {{
                _mm_or_si128(
                    _mm_or_si128(_mm_cmpeq_epi8($a, v_a), _mm_cmpeq_epi8($a, v_b)),
                    _mm_cmpeq_epi8($a, v_c),
                )
            }};
        }
    };
    ($fa:expr, $fb:expr, 128, ) => {
        use std::arch::x86_64::{_mm_cmpeq_epi8, _mm_or_si128, _mm_set1_epi8};
        const A: i8 = $fa;
        const B: i8 = $fb;

        let v_a = _mm_set1_epi8(A);
        let v_b = _mm_set1_epi8(B);

        macro_rules! masking_128 {
            ($a:ident) => {{
                _mm_or_si128(_mm_cmpeq_epi8($a, v_a), _mm_cmpeq_epi8($a, v_b))
            }};
        }
    };
    ($fa:expr, 128, ) => {
        use std::arch::x86_64::{_mm_cmpeq_epi8, _mm_set1_epi8};
        const A: i8 = $fa;

        let v_a = _mm_set1_epi8(A);

        macro_rules! masking_128 {
            ($a:ident) => {{
                _mm_cmpeq_epi8($a, v_a)
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
        use std::arch::x86_64::{_mm_add_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8};
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;
        const TRANSLATION_C: i8 = ::std::i8::MAX - $rc;
        const BELOW_C: i8 = ::std::i8::MAX - ($rc - $lc) - 1;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm_set1_epi8(BELOW_B);
        let v_translation_c = _mm_set1_epi8(TRANSLATION_C);
        let v_below_c = _mm_set1_epi8(BELOW_C);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_or_si128(
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_b), v_below_b),
                    ),
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_c), v_below_c),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
        use std::arch::x86_64::{
            _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;
        const C: i8 = $c;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm_set1_epi8(BELOW_B);
        let v_c = _mm_set1_epi8(C);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_or_si128(
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_b), v_below_b),
                    ),
                    _mm_cmpeq_epi8($a, v_c),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, ) => {
        use std::arch::x86_64::{_mm_add_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8};
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm_set1_epi8(BELOW_B);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_b), v_below_b),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $b:expr, ) => {
        use std::arch::x86_64::{
            _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const B: i8 = $b;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_b = _mm_set1_epi8(B);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                    _mm_cmpeq_epi8($a, v_b),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, ) => {
        use std::arch::x86_64::{_mm_add_epi8, _mm_cmpgt_epi8, _mm_set1_epi8};
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a)
            }};
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Generate mask bodies callback
///
/// Defining exact match or false positive
/// ## The following macros must be defined
/// * `mask_bodies_callback($callback:ident)`
///
macro_rules! _v_escape_mask_bodies_escaping {
    ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
        mask_bodies_callback!(_v_escape_bodies);
    };
    ($fa:expr, $fb:expr, $fc:expr, 128, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($fa:expr, $fb:expr, 128, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($fa:expr, 128, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($la:expr, $ra:expr, $b:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($la:expr, $ra:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _v_escape_escape_avx {
    (($T:ident, $Q:ident, $Q_LEN:ident) $($t:tt ,)+) => {
        #[inline]
        #[target_feature(enable = "avx2")]
        pub unsafe fn escape(bytes: &[u8], fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            #[allow(unused_imports)]
            use std::arch::x86_64::{
                __m256i, _mm256_load_si256, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_or_si256,
            };

            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;

            let mut start = 0;

            macro_rules! mask_bodies_callback {
                ($callback:ident) => {
                    // Format bytes in the mask that starts in the current pointer
                    macro_rules! mask_bodies {
                        ($mask:ident, $at:ident, $cur:ident, $ptr:ident) => {
                            // Calls macro `bodies!` at position `$at + $cur`
                            // of byte `*$ptr` + `$curr` with macro `_v_escape_mask_body!`
                            $callback!($T, $Q, $Q_LEN, $at + $cur, *$ptr.add($cur), start, fmt, bytes, _v_escape_mask_body);

                            // Create binary vector of all zeros except
                            // position `$curr` and xor operation with `$mask`
                            $mask ^= 1 << $cur;
                            // Test vs Check  if `$mask` is empty
                            if $mask == 0 {
                                break;
                            }

                            // Get to the next possible escape character avoiding zeros
                            $cur = $mask.trailing_zeros() as usize;
                        };
                    }
                };
            }

            _v_escape_mask_bodies_escaping!($($t, )+);

            // Macro to write with mask
            macro_rules! write_mask {
                ($mask:ident, $ptr:ident) => {{
                    // Reference to the start of mask
                    let at = _v_escape_sub!($ptr, start_ptr);
                    // Get to the first possible escape character avoiding zeros
                    let mut cur = $mask.trailing_zeros() as usize;

                    loop {
                        // Writing in `$fmt` with `$mask`
                        // The main loop will break when mask == 0
                        mask_bodies!($mask, at, cur, $ptr);
                    }

                    debug_assert_eq!(at, _v_escape_sub!($ptr, start_ptr))
                }};
            }

            // Write a sliced mask
            macro_rules! write_forward {
                ($mask: ident, $align:ident) => {{
                    let at = _v_escape_sub!(ptr, start_ptr);
                    let mut cur = $mask.trailing_zeros() as usize;

                    while cur < $align {
                        mask_bodies!($mask, at, cur, ptr);
                    }

                    debug_assert_eq!(at, _v_escape_sub!(ptr, start_ptr))
                }};
            }

            const M256_VECTOR_SIZE: usize = ::std::mem::size_of::<__m256i>();
            const M256_VECTOR_ALIGN: usize = M256_VECTOR_SIZE - 1;
            const LOOP_SIZE: usize = 4 * M256_VECTOR_SIZE;

            if len < M256_VECTOR_SIZE {
                use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_loadu_si128, _mm_movemask_epi8};

                const M128_VECTOR_SIZE: usize = ::std::mem::size_of::<__m128i>();
                const M128_VECTOR_ALIGN: usize = M128_VECTOR_SIZE - 1;

                if len < M128_VECTOR_SIZE {
                     while ptr < end_ptr {
                         _v_escape_bodies!(
                            $T,
                            $Q,
                            $Q_LEN,
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
                    _v_escape_translations_128!($($t, )+);

                    // Write mask for unaligned elements from the start
                    // of the vector and aligning pointer
                    {
                        // Calculating index of aligned pointer
                        let align = M128_VECTOR_SIZE - (start_ptr as usize & M128_VECTOR_ALIGN);
                        if align < M128_VECTOR_SIZE {
                            let mut mask = {
                                let a = _mm_loadu_si128(ptr as *const __m128i);
                                _mm_movemask_epi8(masking_128!(a))
                            };
                            // Writing mask for unaligned elements
                            if mask != 0 {
                                write_forward!(mask, align);
                            }
                            // Aligning pointer
                            ptr = ptr.add(align);
                        }
                    }
                    // Process all aligned slices with at least one set of length `M128_VECTOR_SIZE`
                    while ptr <= end_ptr.sub(M128_VECTOR_SIZE) {
                        debug_assert_eq!(0, (ptr as usize) % M128_VECTOR_SIZE);
                        let mut mask = {
                            let a = _mm_load_si128(ptr as *const __m128i);
                            _mm_movemask_epi8(masking_128!(a))
                        };

                        if mask != 0 {
                            write_mask!(mask, ptr);
                        }
                        ptr = ptr.add(M128_VECTOR_SIZE);
                    }

                    debug_assert!(end_ptr.sub(M128_VECTOR_SIZE) < ptr);

                    // At this point at most there is less than `M128_VECTOR_SIZE` elements
                    // so the macro `write_mask` is used to the last elements
                    if ptr < end_ptr {
                        debug_assert_eq!(0, (ptr as usize) % M128_VECTOR_SIZE);

                        #[allow(unused_mut)]
                        let mut mask = {
                            let a = _mm_load_si128(ptr as *const __m128i);
                            _mm_movemask_epi8(masking_128!(a))
                        };

                        if mask != 0 {
                            let end = _v_escape_sub!(end_ptr, ptr);
                            write_forward!(mask, end);
                        }
                    }
                }
            } else {
                _v_escape_translations!($($t, )+);

                // Aligning pointer by using `_mm256_loadu_si256` on unaligned bytes.
                {
                    let align = M256_VECTOR_SIZE - (start_ptr as usize & M256_VECTOR_ALIGN);
                    if align < M256_VECTOR_SIZE {
                        let mut mask = {
                            let a = _mm256_loadu_si256(ptr as *const __m256i);
                            _mm256_movemask_epi8(masking!(a))
                        };

                        write_forward!(mask, align);
                        // Aligning pointer
                        ptr = ptr.add(align);
                    }
                }

                debug_assert!(start_ptr <= ptr && start_ptr <= end_ptr.sub(M256_VECTOR_SIZE));

                // Process all aligned slices with at least one set of length `LOOP_SIZE`
                if LOOP_SIZE <= len {
                    while ptr <= end_ptr.sub(LOOP_SIZE) {
                        debug_assert_eq!(0, (ptr as usize) % M256_VECTOR_SIZE);

                        // Using function `_mm256_load_si256` for faster behavior on aligned bytes.
                        // Getting 4 sets of length `M256_VECTOR_SIZE` each (`LOOP_SIZE=4*M256_VECTOR_SIZE`)
                        let cmp_a = {
                            let a = _mm256_load_si256(ptr as *const __m256i);
                            masking!(a)
                        };

                        let cmp_b = {
                            let a = _mm256_load_si256(ptr.add(M256_VECTOR_SIZE) as *const __m256i);
                            masking!(a)
                        };

                        let cmp_c = {
                            let a = _mm256_load_si256(ptr.add(M256_VECTOR_SIZE * 2) as *const __m256i);
                            masking!(a)
                        };

                        let cmp_d = {
                            let a = _mm256_load_si256(ptr.add(M256_VECTOR_SIZE * 3) as *const __m256i);
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
                                write_mask!(mask, ptr);
                            }

                            mask = _mm256_movemask_epi8(cmp_b);
                            if mask != 0 {
                                let ptr = ptr.add(M256_VECTOR_SIZE);
                                write_mask!(mask, ptr);
                            }

                            mask = _mm256_movemask_epi8(cmp_c);
                            if mask != 0 {
                                let ptr = ptr.add(M256_VECTOR_SIZE * 2);
                                write_mask!(mask, ptr);
                            }

                            mask = _mm256_movemask_epi8(cmp_d);
                            if mask != 0 {
                                let ptr = ptr.add(M256_VECTOR_SIZE * 3);
                                write_mask!(mask, ptr);
                            }
                        }

                        ptr = ptr.add(LOOP_SIZE);
                    }
                }

                // When the rest of string has a length greater then `M256_VECTOR_SIZE`
                // but less than `LOOP_SIZE`, we process it `M256_VECTOR_SIZE` bits at
                // a time until there are left less then `M256_VECTOR_SIZE` elements
                while ptr <= end_ptr.sub(M256_VECTOR_SIZE) {
                    debug_assert_eq!(0, (ptr as usize) % M256_VECTOR_SIZE);

                    let mut mask = {
                        let a = _mm256_load_si256(ptr as *const __m256i);
                        _mm256_movemask_epi8(masking!(a))
                    };

                    if mask != 0 {
                        write_mask!(mask, ptr);
                    }
                    ptr = ptr.add(M256_VECTOR_SIZE);
                }

                debug_assert!(end_ptr.sub(M256_VECTOR_SIZE) < ptr);

                // At this point at most there is less then `M256_VECTOR_SIZE` elements
                // so the macro `write_forward` is used to finalize de process
                if ptr < end_ptr {
                    debug_assert_eq!(0, (ptr as usize) % M256_VECTOR_SIZE);

                    let mut mask = {
                        let a = _mm256_load_si256(ptr as *const __m256i);
                        _mm256_movemask_epi8(masking!(a))
                    };

                    if mask != 0 {
                        let end = _v_escape_sub!(end_ptr, ptr);
                        write_forward!(mask, end);
                    }
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
