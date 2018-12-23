/// Generate translations
///
/// Defining character interval from ASCII table to create bit masks from slice to be escaped
/// overflow above in addition
#[macro_export]
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
macro_rules! _v_escape_escape_avx {
    (($T:ident, $Q:ident, $Q_LEN:ident) $($t:tt ,)+) => {
        #[inline]
        #[target_feature(enable = "avx2")]
        pub unsafe fn escape(bytes: &[u8], fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {

            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let mut ptr = start_ptr;
            let mut start = 0;

            // Format bytes in the mask that starts in the current pointer
            macro_rules! mask_bodies {
                ($mask:ident, $at:ident, $cur:ident, $ptr:ident) => {
                    // Calls macro `bodies!` at position `$at + $cur`
                    // of byte `*$ptr` + `$curr` with macro `_v_escape_mask_body!`
                    _v_escape_bodies!($T, $Q, $Q_LEN, $at + $cur, *$ptr.add($cur), start, fmt, bytes, _v_escape_mask_body);

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

            _v_escape_translations!($($t, )+);

            loop_m256_128!(len, ptr, start_ptr, bytes);

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
macro_rules! _v_escape_sized_avx {
    (($S:ident) $($t:tt ,)+) => {
        #[inline]
        #[target_feature(enable = "avx2")]
        pub unsafe fn size(bytes: &[u8]) -> usize {

            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let mut acc = len;
            let mut ptr = start_ptr;

            // Size bytes in the mask that starts in the current pointer
            macro_rules! mask_bodies {
                ($mask:ident, $cur:ident, $ptr:ident) => {

                     _v_escape_size_bodies!($S, acc, *$ptr.add($cur));
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

            // Macro to write with mask
            macro_rules! write_mask {
                ($mask:ident, $ptr:ident) => {{
                    // Get to the first possible escape character avoiding zeros
                    let mut cur = $mask.trailing_zeros() as usize;

                    loop {
                        // The main loop will break when mask == 0
                        mask_bodies!($mask, cur, $ptr);
                    }
                }};
            }

            // Write a sliced mask
            macro_rules! write_forward {
                ($mask: ident, $align:ident) => {{
                    let mut cur = $mask.trailing_zeros() as usize;

                    while cur < $align {
                        mask_bodies!($mask, cur, ptr);
                    }
                }};
            }

            _v_escape_translations!($($t, )+);

            loop_m256_128!(len, ptr, start_ptr, bytes);

            acc
        }
    };
}
