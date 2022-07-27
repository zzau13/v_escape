static V_ESCAPE_CHARS: [u8; 256] = [
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 0u8, 1u8, 2u8, 3u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 4u8, 10u8, 5u8, 6u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 7u8, 10u8, 8u8, 9u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
];
static V_ESCAPE_QUOTES: [&str; 10usize] = [
    "\\#",
    "\\$",
    "\\%",
    "\\&",
    "\\textbackslash{}",
    "\\textasciicircum{}",
    "\\_",
    "\\{",
    "\\}",
    "\\textasciitilde{}",
];
const V_ESCAPE_LEN: usize = 10usize;
#[inline(always)]
fn sub(a: *const u8, b: *const u8) -> usize {
    debug_assert!(b <= a);
    (a as usize) - (b as usize)
}
pub mod scalar {
    use super::*;
    pub struct __Escaped<'a>(&'a [u8]);
    impl<'a> std::fmt::Display for __Escaped<'a> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            unsafe { _escape(self.0, fmt) }
        }
    }
    pub fn escape(s: &str) -> __Escaped {
        __Escaped(s.as_bytes())
    }
    pub unsafe fn _escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let len = bytes.len();
        let start_ptr = bytes.as_ptr();
        let end_ptr = bytes[len..].as_ptr();
        let mut ptr = start_ptr;
        let mut start = 0;
        while ptr < end_ptr {
            let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr as usize) as usize;
            if c < V_ESCAPE_LEN {
                let i = sub(ptr, start_ptr);
                if start < i {
                    fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..i]))?;
                }
                fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                start = i + 1;
            }
            ptr = ptr.offset(1);
        }
        debug_assert!(start <= len);
        if start < len {
            fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..len]))?;
        }
        Ok(())
    }
    #[cfg(feature = "bytes-buf")]
    pub fn b_escape<B: buf_min::Buffer>(bytes: &[u8], fmt: &mut B) {
        unsafe {
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;
            let mut start = 0;
            while ptr < end_ptr {
                let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr as usize) as usize;
                if c < V_ESCAPE_LEN {
                    let i = sub(ptr, start_ptr);
                    if start < i {
                        fmt.extend_from_slice(&bytes[start..i]);
                    }
                    fmt.extend_from_slice((*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes());
                    start = i + 1;
                }
                ptr = ptr.offset(1);
            }
            debug_assert!(start <= len);
            if start < len {
                fmt.extend_from_slice(&bytes[start..]);
            }
        }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub mod ranges {
    pub mod avx {
        use super::super::*;
        #[target_feature(enable = "avx2")]
        pub unsafe fn escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[cfg(target_arch = "x86")]
            use std::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::*;
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;
            let mut start = 0;
            const M256_VECTOR_SIZE: usize = std::mem::size_of::<__m256i>();
            const LOOP_SIZE: usize = 4 * M256_VECTOR_SIZE;
            if len < M256_VECTOR_SIZE {
                const M128_VECTOR_SIZE: usize = std::mem::size_of::<__m128i>();
                const M128_VECTOR_ALIGN: usize = M128_VECTOR_SIZE - 1;
                if len < M128_VECTOR_SIZE {
                    while ptr < end_ptr {
                        let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr as usize) as usize;
                        if c < V_ESCAPE_LEN {
                            let i = sub(ptr, start_ptr);
                            if start < i {
                                fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..i]))?;
                            }
                            fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                            start = i + 1;
                        }
                        ptr = ptr.offset(1);
                    }
                } else {
                    const TRANSLATION_A: i8 = i8::MAX - 38i8;
                    const BELOW_A: i8 = i8::MAX - (38i8 - 35i8) - 1;
                    const TRANSLATION_B: i8 = i8::MAX - 95i8;
                    const BELOW_B: i8 = i8::MAX - (95i8 - 92i8) - 1;
                    const TRANSLATION_C: i8 = i8::MAX - 126i8;
                    const BELOW_C: i8 = i8::MAX - (126i8 - 123i8) - 1;
                    let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm_set1_epi8(BELOW_A);
                    let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
                    let v_below_b = _mm_set1_epi8(BELOW_B);
                    let v_translation_c = _mm_set1_epi8(TRANSLATION_C);
                    let v_below_c = _mm_set1_epi8(BELOW_C);
                    {
                        let align = M128_VECTOR_SIZE - (start_ptr as usize & M128_VECTOR_ALIGN);
                        if align < M128_VECTOR_SIZE {
                            let mut mask = {
                                let a = _mm_loadu_si128(ptr as *const __m128i);
                                _mm_movemask_epi8(_mm_or_si128(
                                    _mm_or_si128(
                                        _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                        _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                                    ),
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                                ))
                            };
                            if mask != 0 {
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                while cur < align {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.write_str(std::str::from_utf8_unchecked(
                                                &bytes[start..i],
                                            ))?;
                                        }
                                        fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                            ptr = ptr.add(align);
                        }
                    }
                    while ptr <= end_ptr.sub(M128_VECTOR_SIZE) {
                        debug_assert_eq!(0, (ptr as usize) % M128_VECTOR_SIZE);
                        let mut mask = {
                            let a = _mm_load_si128(ptr as *const __m128i);
                            _mm_movemask_epi8(_mm_or_si128(
                                _mm_or_si128(
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                                ),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                            ))
                        };
                        if mask != 0 {
                            let at = sub(ptr, start_ptr);
                            let mut cur = mask.trailing_zeros() as usize;
                            loop {
                                let c =
                                    *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                                if c < V_ESCAPE_LEN {
                                    let i = at + cur;
                                    let i = i;
                                    if start < i {
                                        fmt.write_str(std::str::from_utf8_unchecked(
                                            &bytes[start..i],
                                        ))?;
                                    }
                                    fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                    start = i + 1;
                                }
                                mask ^= 1 << cur;
                                if mask == 0 {
                                    break;
                                }
                                cur = mask.trailing_zeros() as usize;
                            }
                            debug_assert_eq!(at, sub(ptr, start_ptr));
                        }
                        ptr = ptr.add(M128_VECTOR_SIZE);
                    }
                    debug_assert!(end_ptr.sub(M128_VECTOR_SIZE) < ptr);
                    if ptr < end_ptr {
                        let d = M128_VECTOR_SIZE - sub(end_ptr, ptr);
                        let mut mask = ({
                            debug_assert_eq!(M128_VECTOR_SIZE, sub(end_ptr, ptr.sub(d)));
                            let a = _mm_loadu_si128(ptr.sub(d) as *const __m128i);
                            _mm_movemask_epi8(_mm_or_si128(
                                _mm_or_si128(
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                                ),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                            ))
                        } as u16)
                            .wrapping_shr(d as u32);
                        if mask != 0 {
                            let at = sub(ptr, start_ptr);
                            let mut cur = mask.trailing_zeros() as usize;
                            loop {
                                let c =
                                    *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                                if c < V_ESCAPE_LEN {
                                    let i = at + cur;
                                    let i = i;
                                    if start < i {
                                        fmt.write_str(std::str::from_utf8_unchecked(
                                            &bytes[start..i],
                                        ))?;
                                    }
                                    fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                    start = i + 1;
                                }
                                mask ^= 1 << cur;
                                if mask == 0 {
                                    break;
                                }
                                cur = mask.trailing_zeros() as usize;
                            }
                            debug_assert_eq!(at, sub(ptr, start_ptr))
                        }
                    }
                }
            } else {
                const TRANSLATION_A: i8 = i8::MAX - 38i8;
                const BELOW_A: i8 = i8::MAX - (38i8 - 35i8) - 1;
                const TRANSLATION_B: i8 = i8::MAX - 95i8;
                const BELOW_B: i8 = i8::MAX - (95i8 - 92i8) - 1;
                const TRANSLATION_C: i8 = i8::MAX - 126i8;
                const BELOW_C: i8 = i8::MAX - (126i8 - 123i8) - 1;
                let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
                let v_below_a = _mm256_set1_epi8(BELOW_A);
                let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
                let v_below_b = _mm256_set1_epi8(BELOW_B);
                let v_translation_c = _mm256_set1_epi8(TRANSLATION_C);
                let v_below_c = _mm256_set1_epi8(BELOW_C);
                {
                    const M256_VECTOR_ALIGN: usize = M256_VECTOR_SIZE - 1;
                    let align = M256_VECTOR_SIZE - (start_ptr as usize & M256_VECTOR_ALIGN);
                    if align < M256_VECTOR_SIZE {
                        let mut mask = {
                            let a = _mm256_loadu_si256(ptr as *const __m256i);
                            _mm256_movemask_epi8(_mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            ))
                        };
                        if mask != 0 {
                            let at = sub(ptr, start_ptr);
                            let mut cur = mask.trailing_zeros() as usize;
                            while cur < align {
                                let c =
                                    *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                                if c < V_ESCAPE_LEN {
                                    let i = at + cur;
                                    let i = i;
                                    if start < i {
                                        fmt.write_str(std::str::from_utf8_unchecked(
                                            &bytes[start..i],
                                        ))?;
                                    }
                                    fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                    start = i + 1;
                                }
                                mask ^= 1 << cur;
                                if mask == 0 {
                                    break;
                                }
                                cur = mask.trailing_zeros() as usize;
                            }
                            debug_assert_eq!(at, sub(ptr, start_ptr))
                        }
                        ptr = ptr.add(align);
                    }
                }
                if LOOP_SIZE <= len {
                    while ptr <= end_ptr.sub(LOOP_SIZE) {
                        debug_assert_eq!(0, (ptr as usize) % M256_VECTOR_SIZE);
                        let cmp_a = {
                            let a = _mm256_load_si256(ptr as *const __m256i);
                            _mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            )
                        };
                        let cmp_b = {
                            let a = _mm256_load_si256(ptr.add(M256_VECTOR_SIZE) as *const __m256i);
                            _mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            )
                        };
                        let cmp_c = {
                            let a =
                                _mm256_load_si256(ptr.add(M256_VECTOR_SIZE * 2) as *const __m256i);
                            _mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            )
                        };
                        let cmp_d = {
                            let a =
                                _mm256_load_si256(ptr.add(M256_VECTOR_SIZE * 3) as *const __m256i);
                            _mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            )
                        };
                        if _mm256_movemask_epi8(_mm256_or_si256(
                            _mm256_or_si256(cmp_a, cmp_b),
                            _mm256_or_si256(cmp_c, cmp_d),
                        )) != 0
                        {
                            let mut mask = _mm256_movemask_epi8(cmp_a);
                            if mask != 0 {
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                loop {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.write_str(std::str::from_utf8_unchecked(
                                                &bytes[start..i],
                                            ))?;
                                        }
                                        fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                            mask = _mm256_movemask_epi8(cmp_b);
                            if mask != 0 {
                                let ptr = ptr.add(M256_VECTOR_SIZE);
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                loop {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.write_str(std::str::from_utf8_unchecked(
                                                &bytes[start..i],
                                            ))?;
                                        }
                                        fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                            mask = _mm256_movemask_epi8(cmp_c);
                            if mask != 0 {
                                let ptr = ptr.add(M256_VECTOR_SIZE * 2);
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                loop {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.write_str(std::str::from_utf8_unchecked(
                                                &bytes[start..i],
                                            ))?;
                                        }
                                        fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                            mask = _mm256_movemask_epi8(cmp_d);
                            if mask != 0 {
                                let ptr = ptr.add(M256_VECTOR_SIZE * 3);
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                loop {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.write_str(std::str::from_utf8_unchecked(
                                                &bytes[start..i],
                                            ))?;
                                        }
                                        fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                        }
                        ptr = ptr.add(LOOP_SIZE);
                    }
                }
                while ptr <= end_ptr.sub(M256_VECTOR_SIZE) {
                    debug_assert_eq!(0, (ptr as usize) % M256_VECTOR_SIZE);
                    let mut mask = {
                        let a = _mm256_load_si256(ptr as *const __m256i);
                        _mm256_movemask_epi8(_mm256_or_si256(
                            _mm256_or_si256(
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_a), v_below_a),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_b), v_below_b),
                            ),
                            _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                        ))
                    };
                    if mask != 0 {
                        let at = sub(ptr, start_ptr);
                        let mut cur = mask.trailing_zeros() as usize;
                        loop {
                            let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                            if c < V_ESCAPE_LEN {
                                let i = at + cur;
                                let i = i;
                                if start < i {
                                    fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..i]))?;
                                }
                                fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                start = i + 1;
                            }
                            mask ^= 1 << cur;
                            if mask == 0 {
                                break;
                            }
                            cur = mask.trailing_zeros() as usize;
                        }
                        debug_assert_eq!(at, sub(ptr, start_ptr))
                    }
                    ptr = ptr.add(M256_VECTOR_SIZE);
                }
                debug_assert!(end_ptr.sub(M256_VECTOR_SIZE) < ptr);
                if ptr < end_ptr {
                    let d = M256_VECTOR_SIZE - sub(end_ptr, ptr);
                    let mut mask = ({
                        debug_assert_eq!(M256_VECTOR_SIZE, sub(end_ptr, ptr.sub(d)), "Over runs");
                        let a = _mm256_loadu_si256(ptr.sub(d) as *const __m256i);
                        _mm256_movemask_epi8(_mm256_or_si256(
                            _mm256_or_si256(
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_a), v_below_a),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_b), v_below_b),
                            ),
                            _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                        ))
                    } as u32)
                        .wrapping_shr(d as u32);
                    if mask != 0 {
                        let at = sub(ptr, start_ptr);
                        let mut cur = mask.trailing_zeros() as usize;
                        loop {
                            let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                            if c < V_ESCAPE_LEN {
                                let i = at + cur;
                                let i = i;
                                if start < i {
                                    fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..i]))?;
                                }
                                fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                start = i + 1;
                            }
                            mask ^= 1 << cur;
                            if mask == 0 {
                                break;
                            }
                            cur = mask.trailing_zeros() as usize;
                        }
                        debug_assert_eq!(at, sub(ptr, start_ptr))
                    }
                }
            }
            debug_assert!(start <= len);
            if start < len {
                fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..len]))?;
            }
            Ok(())
        }
        #[cfg(feature = "bytes-buf")]
        #[target_feature(enable = "avx2")]
        pub unsafe fn b_escape<B: buf_min::Buffer>(bytes: &[u8], fmt: &mut B) {
            #[cfg(target_arch = "x86")]
            use std::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::*;
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;
            let mut start = 0;
            const M256_VECTOR_SIZE: usize = std::mem::size_of::<__m256i>();
            const LOOP_SIZE: usize = 4 * M256_VECTOR_SIZE;
            if len < M256_VECTOR_SIZE {
                const M128_VECTOR_SIZE: usize = std::mem::size_of::<__m128i>();
                const M128_VECTOR_ALIGN: usize = M128_VECTOR_SIZE - 1;
                if len < M128_VECTOR_SIZE {
                    while ptr < end_ptr {
                        let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr as usize) as usize;
                        if c < V_ESCAPE_LEN {
                            let i = sub(ptr, start_ptr);
                            if start < i {
                                fmt.extend_from_slice(&bytes[start..i]);
                            }
                            fmt.extend_from_slice(
                                (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                            );
                            start = i + 1;
                        }
                        ptr = ptr.offset(1);
                    }
                } else {
                    const TRANSLATION_A: i8 = i8::MAX - 38i8;
                    const BELOW_A: i8 = i8::MAX - (38i8 - 35i8) - 1;
                    const TRANSLATION_B: i8 = i8::MAX - 95i8;
                    const BELOW_B: i8 = i8::MAX - (95i8 - 92i8) - 1;
                    const TRANSLATION_C: i8 = i8::MAX - 126i8;
                    const BELOW_C: i8 = i8::MAX - (126i8 - 123i8) - 1;
                    let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm_set1_epi8(BELOW_A);
                    let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
                    let v_below_b = _mm_set1_epi8(BELOW_B);
                    let v_translation_c = _mm_set1_epi8(TRANSLATION_C);
                    let v_below_c = _mm_set1_epi8(BELOW_C);
                    {
                        let align = M128_VECTOR_SIZE - (start_ptr as usize & M128_VECTOR_ALIGN);
                        if align < M128_VECTOR_SIZE {
                            let mut mask = {
                                let a = _mm_loadu_si128(ptr as *const __m128i);
                                _mm_movemask_epi8(_mm_or_si128(
                                    _mm_or_si128(
                                        _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                        _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                                    ),
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                                ))
                            };
                            if mask != 0 {
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                while cur < align {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.extend_from_slice(&bytes[start..i]);
                                        }
                                        fmt.extend_from_slice(
                                            (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                        );
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                            ptr = ptr.add(align);
                        }
                    }
                    while ptr <= end_ptr.sub(M128_VECTOR_SIZE) {
                        debug_assert_eq!(0, (ptr as usize) % M128_VECTOR_SIZE);
                        let mut mask = {
                            let a = _mm_load_si128(ptr as *const __m128i);
                            _mm_movemask_epi8(_mm_or_si128(
                                _mm_or_si128(
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                                ),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                            ))
                        };
                        if mask != 0 {
                            let at = sub(ptr, start_ptr);
                            let mut cur = mask.trailing_zeros() as usize;
                            loop {
                                let c =
                                    *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                                if c < V_ESCAPE_LEN {
                                    let i = at + cur;
                                    let i = i;
                                    if start < i {
                                        fmt.extend_from_slice(&bytes[start..i]);
                                    }
                                    fmt.extend_from_slice(
                                        (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                    );
                                    start = i + 1;
                                }
                                mask ^= 1 << cur;
                                if mask == 0 {
                                    break;
                                }
                                cur = mask.trailing_zeros() as usize;
                            }
                            debug_assert_eq!(at, sub(ptr, start_ptr));
                        }
                        ptr = ptr.add(M128_VECTOR_SIZE);
                    }
                    debug_assert!(end_ptr.sub(M128_VECTOR_SIZE) < ptr);
                    if ptr < end_ptr {
                        let d = M128_VECTOR_SIZE - sub(end_ptr, ptr);
                        let mut mask = ({
                            debug_assert_eq!(M128_VECTOR_SIZE, sub(end_ptr, ptr.sub(d)));
                            let a = _mm_loadu_si128(ptr.sub(d) as *const __m128i);
                            _mm_movemask_epi8(_mm_or_si128(
                                _mm_or_si128(
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                                ),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                            ))
                        } as u16)
                            .wrapping_shr(d as u32);
                        if mask != 0 {
                            let at = sub(ptr, start_ptr);
                            let mut cur = mask.trailing_zeros() as usize;
                            loop {
                                let c =
                                    *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                                if c < V_ESCAPE_LEN {
                                    let i = at + cur;
                                    let i = i;
                                    if start < i {
                                        fmt.extend_from_slice(&bytes[start..i]);
                                    }
                                    fmt.extend_from_slice(
                                        (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                    );
                                    start = i + 1;
                                }
                                mask ^= 1 << cur;
                                if mask == 0 {
                                    break;
                                }
                                cur = mask.trailing_zeros() as usize;
                            }
                            debug_assert_eq!(at, sub(ptr, start_ptr))
                        }
                    }
                }
            } else {
                const TRANSLATION_A: i8 = i8::MAX - 38i8;
                const BELOW_A: i8 = i8::MAX - (38i8 - 35i8) - 1;
                const TRANSLATION_B: i8 = i8::MAX - 95i8;
                const BELOW_B: i8 = i8::MAX - (95i8 - 92i8) - 1;
                const TRANSLATION_C: i8 = i8::MAX - 126i8;
                const BELOW_C: i8 = i8::MAX - (126i8 - 123i8) - 1;
                let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
                let v_below_a = _mm256_set1_epi8(BELOW_A);
                let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
                let v_below_b = _mm256_set1_epi8(BELOW_B);
                let v_translation_c = _mm256_set1_epi8(TRANSLATION_C);
                let v_below_c = _mm256_set1_epi8(BELOW_C);
                {
                    const M256_VECTOR_ALIGN: usize = M256_VECTOR_SIZE - 1;
                    let align = M256_VECTOR_SIZE - (start_ptr as usize & M256_VECTOR_ALIGN);
                    if align < M256_VECTOR_SIZE {
                        let mut mask = {
                            let a = _mm256_loadu_si256(ptr as *const __m256i);
                            _mm256_movemask_epi8(_mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            ))
                        };
                        if mask != 0 {
                            let at = sub(ptr, start_ptr);
                            let mut cur = mask.trailing_zeros() as usize;
                            while cur < align {
                                let c =
                                    *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                                if c < V_ESCAPE_LEN {
                                    let i = at + cur;
                                    let i = i;
                                    if start < i {
                                        fmt.extend_from_slice(&bytes[start..i]);
                                    }
                                    fmt.extend_from_slice(
                                        (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                    );
                                    start = i + 1;
                                }
                                mask ^= 1 << cur;
                                if mask == 0 {
                                    break;
                                }
                                cur = mask.trailing_zeros() as usize;
                            }
                            debug_assert_eq!(at, sub(ptr, start_ptr))
                        }
                        ptr = ptr.add(align);
                    }
                }
                if LOOP_SIZE <= len {
                    while ptr <= end_ptr.sub(LOOP_SIZE) {
                        debug_assert_eq!(0, (ptr as usize) % M256_VECTOR_SIZE);
                        let cmp_a = {
                            let a = _mm256_load_si256(ptr as *const __m256i);
                            _mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            )
                        };
                        let cmp_b = {
                            let a = _mm256_load_si256(ptr.add(M256_VECTOR_SIZE) as *const __m256i);
                            _mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            )
                        };
                        let cmp_c = {
                            let a =
                                _mm256_load_si256(ptr.add(M256_VECTOR_SIZE * 2) as *const __m256i);
                            _mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            )
                        };
                        let cmp_d = {
                            let a =
                                _mm256_load_si256(ptr.add(M256_VECTOR_SIZE * 3) as *const __m256i);
                            _mm256_or_si256(
                                _mm256_or_si256(
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_a),
                                        v_below_a,
                                    ),
                                    _mm256_cmpgt_epi8(
                                        _mm256_add_epi8(a, v_translation_b),
                                        v_below_b,
                                    ),
                                ),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                            )
                        };
                        if _mm256_movemask_epi8(_mm256_or_si256(
                            _mm256_or_si256(cmp_a, cmp_b),
                            _mm256_or_si256(cmp_c, cmp_d),
                        )) != 0
                        {
                            let mut mask = _mm256_movemask_epi8(cmp_a);
                            if mask != 0 {
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                loop {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.extend_from_slice(&bytes[start..i]);
                                        }
                                        fmt.extend_from_slice(
                                            (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                        );
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                            mask = _mm256_movemask_epi8(cmp_b);
                            if mask != 0 {
                                let ptr = ptr.add(M256_VECTOR_SIZE);
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                loop {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.extend_from_slice(&bytes[start..i]);
                                        }
                                        fmt.extend_from_slice(
                                            (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                        );
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                            mask = _mm256_movemask_epi8(cmp_c);
                            if mask != 0 {
                                let ptr = ptr.add(M256_VECTOR_SIZE * 2);
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                loop {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.extend_from_slice(&bytes[start..i]);
                                        }
                                        fmt.extend_from_slice(
                                            (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                        );
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                            mask = _mm256_movemask_epi8(cmp_d);
                            if mask != 0 {
                                let ptr = ptr.add(M256_VECTOR_SIZE * 3);
                                let at = sub(ptr, start_ptr);
                                let mut cur = mask.trailing_zeros() as usize;
                                loop {
                                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize)
                                        as usize;
                                    if c < V_ESCAPE_LEN {
                                        let i = at + cur;
                                        let i = i;
                                        if start < i {
                                            fmt.extend_from_slice(&bytes[start..i]);
                                        }
                                        fmt.extend_from_slice(
                                            (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                        );
                                        start = i + 1;
                                    }
                                    mask ^= 1 << cur;
                                    if mask == 0 {
                                        break;
                                    }
                                    cur = mask.trailing_zeros() as usize;
                                }
                                debug_assert_eq!(at, sub(ptr, start_ptr))
                            }
                        }
                        ptr = ptr.add(LOOP_SIZE);
                    }
                }
                while ptr <= end_ptr.sub(M256_VECTOR_SIZE) {
                    debug_assert_eq!(0, (ptr as usize) % M256_VECTOR_SIZE);
                    let mut mask = {
                        let a = _mm256_load_si256(ptr as *const __m256i);
                        _mm256_movemask_epi8(_mm256_or_si256(
                            _mm256_or_si256(
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_a), v_below_a),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_b), v_below_b),
                            ),
                            _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                        ))
                    };
                    if mask != 0 {
                        let at = sub(ptr, start_ptr);
                        let mut cur = mask.trailing_zeros() as usize;
                        loop {
                            let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                            if c < V_ESCAPE_LEN {
                                let i = at + cur;
                                let i = i;
                                if start < i {
                                    fmt.extend_from_slice(&bytes[start..i]);
                                }
                                fmt.extend_from_slice(
                                    (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                );
                                start = i + 1;
                            }
                            mask ^= 1 << cur;
                            if mask == 0 {
                                break;
                            }
                            cur = mask.trailing_zeros() as usize;
                        }
                        debug_assert_eq!(at, sub(ptr, start_ptr))
                    }
                    ptr = ptr.add(M256_VECTOR_SIZE);
                }
                debug_assert!(end_ptr.sub(M256_VECTOR_SIZE) < ptr);
                if ptr < end_ptr {
                    let d = M256_VECTOR_SIZE - sub(end_ptr, ptr);
                    let mut mask = ({
                        debug_assert_eq!(M256_VECTOR_SIZE, sub(end_ptr, ptr.sub(d)), "Over runs");
                        let a = _mm256_loadu_si256(ptr.sub(d) as *const __m256i);
                        _mm256_movemask_epi8(_mm256_or_si256(
                            _mm256_or_si256(
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_a), v_below_a),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_b), v_below_b),
                            ),
                            _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_c), v_below_c),
                        ))
                    } as u32)
                        .wrapping_shr(d as u32);
                    if mask != 0 {
                        let at = sub(ptr, start_ptr);
                        let mut cur = mask.trailing_zeros() as usize;
                        loop {
                            let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                            if c < V_ESCAPE_LEN {
                                let i = at + cur;
                                let i = i;
                                if start < i {
                                    fmt.extend_from_slice(&bytes[start..i]);
                                }
                                fmt.extend_from_slice(
                                    (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                );
                                start = i + 1;
                            }
                            mask ^= 1 << cur;
                            if mask == 0 {
                                break;
                            }
                            cur = mask.trailing_zeros() as usize;
                        }
                        debug_assert_eq!(at, sub(ptr, start_ptr))
                    }
                }
            }
            debug_assert!(start <= len);
            if start < len {
                fmt.extend_from_slice(&bytes[start..]);
            }
        }
    }
    pub mod sse {
        use super::super::*;
        #[target_feature(enable = "sse2")]
        pub unsafe fn escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[cfg(target_arch = "x86")]
            use std::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::*;
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;
            let mut start = 0;
            const M128_VECTOR_SIZE: usize = std::mem::size_of::<__m128i>();
            const M128_VECTOR_ALIGN: usize = M128_VECTOR_SIZE - 1;
            if len < M128_VECTOR_SIZE {
                while ptr < end_ptr {
                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr as usize) as usize;
                    if c < V_ESCAPE_LEN {
                        let i = sub(ptr, start_ptr);
                        if start < i {
                            fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..i]))?;
                        }
                        fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                        start = i + 1;
                    }
                    ptr = ptr.offset(1);
                }
            } else {
                const TRANSLATION_A: i8 = i8::MAX - 38i8;
                const BELOW_A: i8 = i8::MAX - (38i8 - 35i8) - 1;
                const TRANSLATION_B: i8 = i8::MAX - 95i8;
                const BELOW_B: i8 = i8::MAX - (95i8 - 92i8) - 1;
                const TRANSLATION_C: i8 = i8::MAX - 126i8;
                const BELOW_C: i8 = i8::MAX - (126i8 - 123i8) - 1;
                let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
                let v_below_a = _mm_set1_epi8(BELOW_A);
                let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
                let v_below_b = _mm_set1_epi8(BELOW_B);
                let v_translation_c = _mm_set1_epi8(TRANSLATION_C);
                let v_below_c = _mm_set1_epi8(BELOW_C);
                {
                    let align = M128_VECTOR_SIZE - (start_ptr as usize & M128_VECTOR_ALIGN);
                    if align < M128_VECTOR_SIZE {
                        let mut mask = {
                            let a = _mm_loadu_si128(ptr as *const __m128i);
                            _mm_movemask_epi8(_mm_or_si128(
                                _mm_or_si128(
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                                ),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                            ))
                        };
                        if mask != 0 {
                            let at = sub(ptr, start_ptr);
                            let mut cur = mask.trailing_zeros() as usize;
                            while cur < align {
                                let c =
                                    *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                                if c < V_ESCAPE_LEN {
                                    let i = at + cur;
                                    let i = i;
                                    if start < i {
                                        fmt.write_str(std::str::from_utf8_unchecked(
                                            &bytes[start..i],
                                        ))?;
                                    }
                                    fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                    start = i + 1;
                                }
                                mask ^= 1 << cur;
                                if mask == 0 {
                                    break;
                                }
                                cur = mask.trailing_zeros() as usize;
                            }
                            debug_assert_eq!(at, sub(ptr, start_ptr))
                        }
                        ptr = ptr.add(align);
                    }
                }
                while ptr <= end_ptr.sub(M128_VECTOR_SIZE) {
                    debug_assert_eq!(0, (ptr as usize) % M128_VECTOR_SIZE);
                    let mut mask = {
                        let a = _mm_load_si128(ptr as *const __m128i);
                        _mm_movemask_epi8(_mm_or_si128(
                            _mm_or_si128(
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                            ),
                            _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                        ))
                    };
                    if mask != 0 {
                        let at = sub(ptr, start_ptr);
                        let mut cur = mask.trailing_zeros() as usize;
                        loop {
                            let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                            if c < V_ESCAPE_LEN {
                                let i = at + cur;
                                let i = i;
                                if start < i {
                                    fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..i]))?;
                                }
                                fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                start = i + 1;
                            }
                            mask ^= 1 << cur;
                            if mask == 0 {
                                break;
                            }
                            cur = mask.trailing_zeros() as usize;
                        }
                        debug_assert_eq!(at, sub(ptr, start_ptr));
                    }
                    ptr = ptr.add(M128_VECTOR_SIZE);
                }
                debug_assert!(end_ptr.sub(M128_VECTOR_SIZE) < ptr);
                if ptr < end_ptr {
                    let d = M128_VECTOR_SIZE - sub(end_ptr, ptr);
                    let mut mask = ({
                        debug_assert_eq!(M128_VECTOR_SIZE, sub(end_ptr, ptr.sub(d)));
                        let a = _mm_loadu_si128(ptr.sub(d) as *const __m128i);
                        _mm_movemask_epi8(_mm_or_si128(
                            _mm_or_si128(
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                            ),
                            _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                        ))
                    } as u16)
                        .wrapping_shr(d as u32);
                    if mask != 0 {
                        let at = sub(ptr, start_ptr);
                        let mut cur = mask.trailing_zeros() as usize;
                        loop {
                            let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                            if c < V_ESCAPE_LEN {
                                let i = at + cur;
                                let i = i;
                                if start < i {
                                    fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..i]))?;
                                }
                                fmt.write_str(*V_ESCAPE_QUOTES.as_ptr().add(c as usize))?;
                                start = i + 1;
                            }
                            mask ^= 1 << cur;
                            if mask == 0 {
                                break;
                            }
                            cur = mask.trailing_zeros() as usize;
                        }
                        debug_assert_eq!(at, sub(ptr, start_ptr))
                    }
                }
            }
            debug_assert!(start <= len);
            if start < len {
                fmt.write_str(std::str::from_utf8_unchecked(&bytes[start..len]))?;
            }
            Ok(())
        }
        #[cfg(feature = "bytes-buf")]
        #[target_feature(enable = "sse2")]
        pub unsafe fn b_escape<B: buf_min::Buffer>(bytes: &[u8], fmt: &mut B) {
            #[cfg(target_arch = "x86")]
            use std::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::*;
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;
            let mut start = 0;
            const M128_VECTOR_SIZE: usize = std::mem::size_of::<__m128i>();
            const M128_VECTOR_ALIGN: usize = M128_VECTOR_SIZE - 1;
            if len < M128_VECTOR_SIZE {
                while ptr < end_ptr {
                    let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr as usize) as usize;
                    if c < V_ESCAPE_LEN {
                        let i = sub(ptr, start_ptr);
                        if start < i {
                            fmt.extend_from_slice(&bytes[start..i]);
                        }
                        fmt.extend_from_slice(
                            (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                        );
                        start = i + 1;
                    }
                    ptr = ptr.offset(1);
                }
            } else {
                const TRANSLATION_A: i8 = i8::MAX - 38i8;
                const BELOW_A: i8 = i8::MAX - (38i8 - 35i8) - 1;
                const TRANSLATION_B: i8 = i8::MAX - 95i8;
                const BELOW_B: i8 = i8::MAX - (95i8 - 92i8) - 1;
                const TRANSLATION_C: i8 = i8::MAX - 126i8;
                const BELOW_C: i8 = i8::MAX - (126i8 - 123i8) - 1;
                let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
                let v_below_a = _mm_set1_epi8(BELOW_A);
                let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
                let v_below_b = _mm_set1_epi8(BELOW_B);
                let v_translation_c = _mm_set1_epi8(TRANSLATION_C);
                let v_below_c = _mm_set1_epi8(BELOW_C);
                {
                    let align = M128_VECTOR_SIZE - (start_ptr as usize & M128_VECTOR_ALIGN);
                    if align < M128_VECTOR_SIZE {
                        let mut mask = {
                            let a = _mm_loadu_si128(ptr as *const __m128i);
                            _mm_movemask_epi8(_mm_or_si128(
                                _mm_or_si128(
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                    _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                                ),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                            ))
                        };
                        if mask != 0 {
                            let at = sub(ptr, start_ptr);
                            let mut cur = mask.trailing_zeros() as usize;
                            while cur < align {
                                let c =
                                    *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                                if c < V_ESCAPE_LEN {
                                    let i = at + cur;
                                    let i = i;
                                    if start < i {
                                        fmt.extend_from_slice(&bytes[start..i]);
                                    }
                                    fmt.extend_from_slice(
                                        (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                    );
                                    start = i + 1;
                                }
                                mask ^= 1 << cur;
                                if mask == 0 {
                                    break;
                                }
                                cur = mask.trailing_zeros() as usize;
                            }
                            debug_assert_eq!(at, sub(ptr, start_ptr))
                        }
                        ptr = ptr.add(align);
                    }
                }
                while ptr <= end_ptr.sub(M128_VECTOR_SIZE) {
                    debug_assert_eq!(0, (ptr as usize) % M128_VECTOR_SIZE);
                    let mut mask = {
                        let a = _mm_load_si128(ptr as *const __m128i);
                        _mm_movemask_epi8(_mm_or_si128(
                            _mm_or_si128(
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                            ),
                            _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                        ))
                    };
                    if mask != 0 {
                        let at = sub(ptr, start_ptr);
                        let mut cur = mask.trailing_zeros() as usize;
                        loop {
                            let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                            if c < V_ESCAPE_LEN {
                                let i = at + cur;
                                let i = i;
                                if start < i {
                                    fmt.extend_from_slice(&bytes[start..i]);
                                }
                                fmt.extend_from_slice(
                                    (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                );
                                start = i + 1;
                            }
                            mask ^= 1 << cur;
                            if mask == 0 {
                                break;
                            }
                            cur = mask.trailing_zeros() as usize;
                        }
                        debug_assert_eq!(at, sub(ptr, start_ptr));
                    }
                    ptr = ptr.add(M128_VECTOR_SIZE);
                }
                debug_assert!(end_ptr.sub(M128_VECTOR_SIZE) < ptr);
                if ptr < end_ptr {
                    let d = M128_VECTOR_SIZE - sub(end_ptr, ptr);
                    let mut mask = ({
                        debug_assert_eq!(M128_VECTOR_SIZE, sub(end_ptr, ptr.sub(d)));
                        let a = _mm_loadu_si128(ptr.sub(d) as *const __m128i);
                        _mm_movemask_epi8(_mm_or_si128(
                            _mm_or_si128(
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
                                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_b), v_below_b),
                            ),
                            _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_c), v_below_c),
                        ))
                    } as u16)
                        .wrapping_shr(d as u32);
                    if mask != 0 {
                        let at = sub(ptr, start_ptr);
                        let mut cur = mask.trailing_zeros() as usize;
                        loop {
                            let c = *V_ESCAPE_CHARS.as_ptr().add(*ptr.add(cur) as usize) as usize;
                            if c < V_ESCAPE_LEN {
                                let i = at + cur;
                                let i = i;
                                if start < i {
                                    fmt.extend_from_slice(&bytes[start..i]);
                                }
                                fmt.extend_from_slice(
                                    (*V_ESCAPE_QUOTES.as_ptr().add(c as usize)).as_bytes(),
                                );
                                start = i + 1;
                            }
                            mask ^= 1 << cur;
                            if mask == 0 {
                                break;
                            }
                            cur = mask.trailing_zeros() as usize;
                        }
                        debug_assert_eq!(at, sub(ptr, start_ptr))
                    }
                }
            }
            debug_assert!(start <= len);
            if start < len {
                fmt.extend_from_slice(&bytes[start..]);
            }
        }
    }
}
pub struct VLatexescape<'a> {
    bytes: &'a [u8],
}
impl<'a> VLatexescape<'a> {
    #[inline]
    pub fn new(bytes: &[u8]) -> VLatexescape {
        VLatexescape { bytes }
    }
}
impl<'a> From<&'a str> for VLatexescape<'a> {
    #[inline]
    fn from(s: &str) -> VLatexescape {
        VLatexescape {
            bytes: s.as_bytes(),
        }
    }
}
#[inline]
pub fn escape(s: &str) -> VLatexescape {
    VLatexescape::from(s)
}
impl<'a> std::fmt::Display for VLatexescape<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_unsafe)]
        unsafe {
            _escape(self.bytes, fmt)
        }
    }
}
/// Escape byte slice to `Buffer`
///
/// # SIGILL
/// Can produce **SIGILL** if compile with `sse2` or `avx2` and execute without they
/// Because not exist way to build multiple static allocations by type
/// And it's very expensive check it in runtime
/// https://github.com/rust-lang/rust/issues/57775
#[cfg(feature = "bytes-buf")]
#[inline]
pub fn b_escape<B: buf_min::Buffer>(s: &[u8], buf: &mut B) {
    #[allow(unused_unsafe)]
    unsafe {
        _b_escape(s, buf)
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline(always)]
fn _escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use std::fmt::{self, Formatter};
    use std::mem;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static mut FN: fn(&[u8], &mut Formatter) -> fmt::Result = detect;
    fn detect(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
        let fun = if is_x86_feature_detected!("avx2") {
            ranges::avx::escape as usize
        } else if is_x86_feature_detected!("sse2") {
            ranges::sse::escape as usize
        } else {
            scalar::_escape as usize
        };
        let slot = unsafe { &*(&FN as *const _ as *const AtomicUsize) };
        slot.store(fun, Ordering::Relaxed);
        unsafe {
            mem::transmute::<usize, fn(&[u8], &mut Formatter) -> fmt::Result>(fun)(bytes, fmt)
        }
    }
    unsafe {
        let slot = &*(&FN as *const _ as *const AtomicUsize);
        let fun = slot.load(Ordering::Relaxed);
        mem::transmute::<usize, fn(&[u8], &mut Formatter) -> fmt::Result>(fun)(bytes, fmt)
    }
}
#[inline(always)]
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
fn _escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    scalar::_escape(bytes, fmt)
}
#[inline(always)]
#[cfg(feature = "bytes-buf")]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub unsafe fn _b_escape<B: buf_min::Buffer>(bytes: &[u8], buf: &mut B) {
    #[cfg(not(v_escape_avx))]
    {
        #[cfg(not(v_escape_sse))]
        {
            scalar::b_escape(bytes, buf)
        }
        #[cfg(v_escape_sse)]
        {
            ranges::sse::b_escape(bytes, buf)
        }
    }
    #[cfg(v_escape_avx)]
    {
        ranges::avx::b_escape(bytes, buf)
    }
}
#[inline(always)]
#[cfg(feature = "bytes-buf")]
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
pub unsafe fn _b_escape<B: buf_min::Buffer>(bytes: &[u8], buf: &mut B) {
    scalar::b_escape(bytes, buf)
}
