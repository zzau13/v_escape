use std::fmt::{self, Formatter};
use std::mem::size_of;
use std::str;
use std::arch::x86_64::{__m128i, _mm_cmpestrm, _mm_extract_epi16, _mm_lddqu_si128, _mm_load_si128,
                        _mm_loadu_si128, _mm_setr_epi8};

use utils::*;

const VECTOR_SIZE: usize = size_of::<__m128i>();
const VECTOR_ALIGN: usize = VECTOR_SIZE - 1;
const NEEDLE_LEN: i32 = 6;

#[target_feature(enable = "sse4.2")]
pub unsafe fn escape(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
    #[rustfmt::skip]
    let needle = _mm_setr_epi8(
        b'<' as i8, b'>' as i8, b'&' as i8, b'"' as i8,
        b'\'' as i8, b'/' as i8, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    );

    let len = bytes.len();
    let start_ptr = bytes.as_ptr();
    let mut ptr = start_ptr;
    let mut start = 0;

    // Macro to write with mask
    macro_rules! write_mask {
        ($mask:ident) => {{
            // Reference to the start of mask
            let at = sub(ptr, start_ptr);
            // Get to the first possible escape character avoiding zeros
            let mut cur = $mask.trailing_zeros() as usize;

            loop {
                // Writing in `$fmt` with `$mask`
                // The main loop will break when mask == 0
                debug_assert_ne!(TABLE[*ptr.add(cur) as usize], QUOTES_LEN as u8);
                mask_body!(at + cur, start, fmt, bytes, QUOTES[TABLE[*ptr.add(cur) as usize] as usize]);

                // Create binary vector of all zeros except
                // position `$curr` and xor operation with `$mask`
                $mask ^= 1 << cur;
                // Test vs Check  if `$mask` is empty
                if $mask == 0 {
                    break;
                }

                // Get to the next possible escape character avoiding zeros
                cur = $mask.trailing_zeros() as usize;
            }

            debug_assert_eq!(at, sub(ptr, start_ptr))
        }};
    }

    if len < VECTOR_SIZE {
        let a = _mm_lddqu_si128(ptr as *const __m128i);
        let cmp = _mm_cmpestrm(needle, NEEDLE_LEN, a, len as i32, 0);
        let mut mask = _mm_extract_epi16(cmp, 0) as i16;

        if mask != 0 {
            write_mask!(mask);
        }
    } else {
        let end_ptr = bytes[len..].as_ptr();

        {
            let align = VECTOR_SIZE - (start_ptr as usize & VECTOR_ALIGN);
            if align < VECTOR_SIZE {
                let a = _mm_loadu_si128(ptr as *const __m128i);
                let cmp = _mm_cmpestrm(needle, NEEDLE_LEN, a, align as i32, 0);
                let mut mask = _mm_extract_epi16(cmp, 0) as i16;

                if mask != 0 {
                    write_mask!(mask);
                }
                ptr = ptr.add(align);

                debug_assert!(start <= sub(ptr, start_ptr));
            }
        }

        while ptr <= end_ptr.sub(VECTOR_SIZE) {
            debug_assert_eq!(0, (ptr as usize) % VECTOR_SIZE);

            let a = _mm_load_si128(ptr as *const __m128i);
            let cmp = _mm_cmpestrm(needle, NEEDLE_LEN, a, VECTOR_SIZE as i32, 0);
            let mut mask = _mm_extract_epi16(cmp, 0) as u16;

            if mask != 0 {
                write_mask!(mask);
            }
            ptr = ptr.add(VECTOR_SIZE);

            debug_assert!(start <= sub(ptr, start_ptr));
        }

        debug_assert!(end_ptr.sub(VECTOR_SIZE) < ptr);

        if ptr < end_ptr {
            debug_assert_eq!(0, (ptr as usize) % VECTOR_SIZE);

            let end = sub(end_ptr, ptr);
            let a = _mm_load_si128(ptr as *const __m128i);
            let cmp = _mm_cmpestrm(needle, NEEDLE_LEN, a, end as i32, 0);
            let mut mask = _mm_extract_epi16(cmp, 0) as u16;

            if mask != 0 {
                write_mask!(mask);
            }
        }
    }

    // Write since start to the end of the slice
    debug_assert!(start <= len);
    if start < len {
        fmt.write_str(str::from_utf8_unchecked(&bytes[start..len]))?;
    }

    Ok(())
}

// TODO: matcher target_feature = "sse4.2"
#[cfg(all(test, target_arch = "x86_64", not(target_os = "windows"), v_htmlescape_sse))]
mod test {
    use super::*;

    new_escape!(SSEscape, escape);

    #[test]
    fn test_escape() {
        let empty = "";
        assert_eq!(SSEscape::new(empty.as_bytes()).to_string(), empty);

        assert_eq!(SSEscape::new("".as_bytes()).to_string(), "");
        assert_eq!(SSEscape::new("<&>".as_bytes()).to_string(), "&lt;&amp;&gt;");
        assert_eq!(SSEscape::new("bar&".as_bytes()).to_string(), "bar&amp;");
        assert_eq!(SSEscape::new("<foo".as_bytes()).to_string(), "&lt;foo");
        assert_eq!(SSEscape::new("bar&h".as_bytes()).to_string(), "bar&amp;h");
        assert_eq!(
            SSEscape::new("// my <html> is \"unsafe\" & should be 'escaped'".repeat(10_000).as_bytes()).to_string(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; should be &#x27;escaped&#x27;".repeat(10_000)
        );
    }
}
