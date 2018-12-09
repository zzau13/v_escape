use std::{
    arch::x86_64::{__m256i, _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8,
                   _mm256_lddqu_si256, _mm256_load_si256, _mm256_loadu_si256,
                   _mm256_movemask_epi8, _mm256_or_si256, _mm256_set1_epi8},
    fmt::{self, Formatter},
    i8,
    mem::size_of,
    str,
};

use utils::*;

// Defining character interval from ASCII table to create bit masks from slice to be escaped
// overflow above in addition
const TRANSLATION_A: i8 = i8::MAX - 39;
const BELOW_A: i8 = i8::MAX - (39 - 34) - 1;
const TRANSLATION_B: i8 = i8::MAX - 62;
const BELOW_B: i8 = i8::MAX - (62 - 60) - 1;
const C: i8 = 47;

const VECTOR_SIZE: usize = size_of::<__m256i>();
const VECTOR_ALIGN: usize = VECTOR_SIZE - 1;
const LOOP_SIZE: usize = 4 * VECTOR_SIZE;

// Main loop DI with macros:
// write_mask(mask: {integer}, ptr: *const u8)
// write_forward(mask: {integer}, until: usize})
// masking(a: __m256i) -> __m256i masking at i8
// TODO: document in detail
macro_rules! loops {
    ($len: ident, $ptr: ident, $start_ptr: ident, $bytes: ident) => {{
        if $len < VECTOR_SIZE {
            let mut mask = {
                let a = _mm256_lddqu_si256($ptr as *const __m256i);
                _mm256_movemask_epi8(masking!(a))
            };

            write_forward!(mask, $len);
        } else {
            let end_ptr = $bytes[$len..].as_ptr();

            {
                let align = VECTOR_SIZE - ($start_ptr as usize & VECTOR_ALIGN);
                if align < VECTOR_SIZE {
                    let mut mask = {
                        let a = _mm256_loadu_si256($ptr as *const __m256i);
                        _mm256_movemask_epi8(masking!(a))
                    };

                    write_forward!(mask, align);
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

                    // Adjust the four masks in two from right to left.
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

                write_forward!(mask, end);
            }
        }
    }};
}

/// Html escape formatter
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn escape(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
    let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
    let v_below_a = _mm256_set1_epi8(BELOW_A);
    let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
    let v_below_b = _mm256_set1_epi8(BELOW_B);
    let v_c = _mm256_set1_epi8(C);

    let len = bytes.len();
    let start_ptr = bytes.as_ptr();
    let mut ptr = start_ptr;
    let mut start = 0;


    // Format bytes in the mask that starts in the current pointer
    macro_rules! mask_bodies {
        ($mask:ident, $at:ident, $cur:ident, $ptr:ident) => {
            // Calls macro `bodies!` at position `$at + $cur`
            // of byte `*$ptr` + `$curr` with macro `mask_body!`
            bodies!($at + $cur, *$ptr.add($cur), start, fmt, bytes, mask_body);

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
            let at = sub($ptr, start_ptr);
            // Get to the first possible escape character avoiding zeros
            let mut cur = $mask.trailing_zeros() as usize;

            loop {
                // Writing in `$fmt` with `$mask`
                // The main loop will break when mask == 0
                mask_bodies!($mask, at, cur, $ptr);
            }

            debug_assert_eq!(at, sub($ptr, start_ptr))
        }};
    }

    // Write a sliced mask
    macro_rules! write_forward {
        ($mask: ident, $align:ident) => {{
            if $mask != 0 {
                let at = sub(ptr, start_ptr);
                let mut cur = $mask.trailing_zeros() as usize;

                while cur < $align {
                    mask_bodies!($mask, at, cur, ptr);
                }

                debug_assert_eq!(at, sub(ptr, start_ptr))
            }
        }};
    }

    // Mask a __m256 with ranges 34-39, 47, 60-62
    macro_rules! masking {
        ($a:ident) => {{
            _mm256_or_si256(
                _mm256_or_si256(
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_b), v_below_b)
                ),
                _mm256_cmpeq_epi8($a, v_c)
            )
        }};
    }

    loops!(len, ptr, start_ptr, bytes);

    // Write since start to the end of the slice
    debug_assert!(start <= len);
    if start < len {
        fmt.write_str(str::from_utf8_unchecked(&bytes[start..len]))?;
    }

    Ok(())
}

/// Length of slice after escape
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn size(bytes: &[u8]) -> usize {
    let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
    let v_below_a = _mm256_set1_epi8(BELOW_A);
    let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
    let v_below_b = _mm256_set1_epi8(BELOW_B);
    let v_c = _mm256_set1_epi8(C);

    let len = bytes.len();
    let start_ptr = bytes.as_ptr();
    let mut acc = len;
    let mut ptr = start_ptr;

    // Size bytes in the mask that starts in the current pointer
    macro_rules! mask_bodies {
        ($mask:ident, $cur:ident, $ptr:ident) => {

            size_bodies!(acc, *$ptr.add($cur));
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
            if $mask != 0 {
                let mut cur = $mask.trailing_zeros() as usize;

                while cur < $align {
                    mask_bodies!($mask, cur, ptr);
                }
            }
        }};
    }

    // Mask a __m256 with ranges 34-39, 47, 60-62
    macro_rules! masking {
        ($a:ident) => {{
            _mm256_or_si256(
                _mm256_or_si256(
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_b), v_below_b)
                ),
                _mm256_cmpeq_epi8($a, v_c)
            )
        }};
    }

    loops!(len, ptr, start_ptr, bytes);

    acc
}

// TODO: matcher target_feature = "avx2"
#[cfg(all(test, target_arch = "x86_64", not(target_os = "windows"), v_htmlescape_avx))]
mod test {
    use super::*;

    new_escape!(AEscape, escape, size);

    #[test]
    fn test_escape() {
        let empty = "";
        assert_eq!(AEscape::new(empty.as_bytes()).to_string(), empty);

        assert_eq!(AEscape::new("".as_bytes()).to_string(), "");
        assert_eq!(AEscape::new("<&>".as_bytes()).to_string(), "&lt;&amp;&gt;");
        assert_eq!(AEscape::new("bar&".as_bytes()).to_string(), "bar&amp;");
        assert_eq!(AEscape::new("<foo".as_bytes()).to_string(), "&lt;foo");
        assert_eq!(AEscape::new("bar&h".as_bytes()).to_string(), "bar&amp;h");
        assert_eq!(
            AEscape::new("// my <html> is \"unsafe\" & should be 'escaped'".repeat(10_000).as_bytes()).to_string(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; should be &#x27;escaped&#x27;".repeat(10_000)
        );
    }

    #[test]
    fn test_size() {
        let empty = "";
        assert_eq!(AEscape::new(empty.as_bytes()).size(), empty.len());

        assert_eq!(AEscape::new("".as_bytes()).size(), 0);
        assert_eq!(AEscape::new("<&>".as_bytes()).size(), "&lt;&amp;&gt;".len());
        assert_eq!(AEscape::new("bar&".as_bytes()).size(), "bar&amp;".len());
        assert_eq!(AEscape::new("<foo".as_bytes()).size(), "&lt;foo".len());
        assert_eq!(AEscape::new("bar&h".as_bytes()).size(), "bar&amp;h".len());
        assert_eq!(
            AEscape::new("// my <html> is \"unsafe\" & should be 'escaped'".repeat(10_000).as_bytes()).size(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; should be &#x27;escaped&#x27;".repeat(10_000).len()
        );
    }
}
