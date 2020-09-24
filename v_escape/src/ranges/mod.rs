#[macro_use]
mod avx;
#[macro_use]
mod sse;
#[macro_use]
mod switch;

#[macro_export]
#[doc(hidden)]
macro_rules! escape_ranges {
    (avx2 $($t:tt)+) => {
        #[inline]
        #[target_feature(enable = "avx2")]
        v_escape::escape_ranges!(impl v_escape::loop_range_switch_avx2 where $($t)+);
    };
    (sse2 $($t:tt)+) => {
        #[inline]
        #[target_feature(enable = "sse2")]
        v_escape::escape_ranges!(impl v_escape::loop_range_switch_sse2 where $($t)+);
    };
    (impl $loops:path where ($T:ident, $Q:ident, $Q_LEN:ident) $($t:tt)+) => {
        pub unsafe fn escape(bytes: &[u8], fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {

            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;

            let mut start = 0;

            macro_rules! mask_bodies_callback {
                ($callback:path) => {
                    // Format bytes in the mask that starts in the current pointer
                    macro_rules! mask_bodies {
                        ($mask:ident, $at:ident, $cur:ident, $ptr:ident) => {
                            // Calls macro `bodies!` at position `$at + $cur`
                            // of byte `*$ptr` + `$curr` with macro `v_escape::mask_body!`
                            $callback!($T, $Q, $Q_LEN, $at + $cur, *$ptr.add($cur), start, fmt, bytes, v_escape::mask_body);

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

            v_escape::mask_bodies_escaping!($($t)+);

            // Macro to write with mask
            macro_rules! write_mask {
                ($mask:ident, $ptr:ident) => {{
                    // Reference to the start of mask
                    let at = v_escape::sub!($ptr, start_ptr);
                    // Get to the first possible escape character avoiding zeros
                    let mut cur = $mask.trailing_zeros() as usize;

                    loop {
                        // Writing in `$fmt` with `$mask`
                        // The main loop will break when mask == 0
                        mask_bodies!($mask, at, cur, $ptr);
                    }

                    debug_assert_eq!(at, v_escape::sub!($ptr, start_ptr))
                }};
            }

            // Write a sliced mask
            macro_rules! write_forward {
                ($mask: ident, $align:ident) => {{
                    let at = v_escape::sub!(ptr, start_ptr);
                    let mut cur = $mask.trailing_zeros() as usize;

                    while cur < $align {
                        mask_bodies!($mask, at, cur, ptr);
                    }

                    debug_assert_eq!(at, v_escape::sub!(ptr, start_ptr))
                }};
            }

            macro_rules! fallback_callback {
                (default) => {
                    macro_rules! fallback {
                        () => {
                            while ptr < end_ptr {
                                v_escape::bodies!(
                                    $T,
                                    $Q,
                                    $Q_LEN,
                                    v_escape::sub!(ptr, start_ptr),
                                    *ptr,
                                    start,
                                    fmt,
                                    bytes,
                                    v_escape::mask_body
                                );
                                ptr = ptr.offset(1);
                            }
                        };
                    }
                };
                (one) => {
                    macro_rules! fallback {
                        () => {
                            while ptr < end_ptr {
                                if *ptr == $T {
                                    v_escape::bodies_exact_one!(
                                        $T,
                                        $Q,
                                        $Q_LEN,
                                        v_escape::sub!(ptr, start_ptr),
                                        *ptr,
                                        start,
                                        fmt,
                                        bytes,
                                        v_escape::mask_body
                                    );
                                }
                                ptr = ptr.offset(1);
                            }
                        };
                    }
                };
            }

            v_escape::fallback_escaping!($($t)+);

            $loops!((len, ptr, start_ptr, end_ptr) $($t)+);

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
#[doc(hidden)]
macro_rules! escape_ranges_ptr {
    (avx2 $($t:tt)+) => {
        #[inline]
        #[target_feature(enable = "avx2")]
        v_escape::escape_ranges_ptr!(impl v_escape::loop_range_switch_avx2 where $($t)+);
    };
    (sse2 $($t:tt)+) => {
        #[inline]
        #[target_feature(enable = "sse2")]
        v_escape::escape_ranges_ptr!(impl v_escape::loop_range_switch_sse2 where $($t)+);
    };
    (impl $loops:path where ($T:ident, $Q:ident, $Q_LEN:ident) $($t:tt)+) => {
        pub unsafe fn f_escape(bytes: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            let mut buf_cur = 0;

            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;

            let mut start = 0;

            macro_rules! mask_bodies_callback {
                ($callback:path) => {
                    // Format bytes in the mask that starts in the current pointer
                    macro_rules! mask_bodies {
                        ($mask:ident, $at:ident, $cur:ident, $ptr:ident) => {
                            // Calls macro `bodies!` at position `$at + $cur`
                            // of byte `*$ptr` + `$curr` with macro `v_escape::mask_body!`
                            $callback!($T, $Q, $Q_LEN, $at + $cur, *$ptr.add($cur), start, buf_cur, buf, start_ptr, v_escape::mask_body_ptr);

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

            v_escape::mask_bodies_escaping_ptr!($($t)+);

            // Macro to write with mask
            macro_rules! write_mask {
                ($mask:ident, $ptr:ident) => {{
                    // Reference to the start of mask
                    let at = v_escape::sub!($ptr, start_ptr);
                    // Get to the first possible escape character avoiding zeros
                    let mut cur = $mask.trailing_zeros() as usize;

                    loop {
                        // Writing in `$fmt` with `$mask`
                        // The main loop will break when mask == 0
                        mask_bodies!($mask, at, cur, $ptr);
                    }

                    debug_assert_eq!(at, v_escape::sub!($ptr, start_ptr))
                }};
            }

            // Write a sliced mask
            macro_rules! write_forward {
                ($mask: ident, $align:ident) => {{
                    let at = v_escape::sub!(ptr, start_ptr);
                    let mut cur = $mask.trailing_zeros() as usize;

                    while cur < $align {
                        mask_bodies!($mask, at, cur, ptr);
                    }

                    debug_assert_eq!(at, v_escape::sub!(ptr, start_ptr))
                }};
            }

            macro_rules! fallback_callback {
                (default) => {
                    macro_rules! fallback {
                        () => {
                            while ptr < end_ptr {
                                v_escape::bodies_ptr!(
                                    $T,
                                    $Q,
                                    $Q_LEN,
                                    v_escape::sub!(ptr, start_ptr),
                                    *ptr,
                                    start,
                                    buf_cur,
                                    buf,
                                    start_ptr,
                                    v_escape::mask_body_ptr
                                );
                                ptr = ptr.offset(1);
                            }
                        };
                    }
                };
                (one) => {
                    macro_rules! fallback {
                        () => {
                            while ptr < end_ptr {
                                if *ptr == $T {
                                    v_escape::bodies_exact_one_ptr!(
                                        $T,
                                        $Q,
                                        $Q_LEN,
                                        v_escape::sub!(ptr, start_ptr),
                                        *ptr,
                                        start,
                                        buf_cur,
                                        buf,
                                        start_ptr,
                                        v_escape::mask_body_ptr
                                    );
                                }
                                ptr = ptr.offset(1);
                            }
                        };
                    }
                };
            }

            v_escape::fallback_escaping!($($t)+);

            $loops!((len, ptr, start_ptr, end_ptr) $($t)+);

            // Write since start to the end of the slice
            debug_assert!(start <= len);
            if start < len {
                let len = len - start;
                v_escape::write_ptr!(buf_cur, buf, start_ptr.add(start), len);
            }

            Some(buf_cur)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! escape_ranges_bytes {
    (avx2 $($t:tt)+) => {
        #[inline]
        #[target_feature(enable = "avx2")]
        v_escape::escape_ranges_bytes!(impl v_escape::loop_range_switch_avx2 where $($t)+);
    };
    (sse2 $($t:tt)+) => {
        #[inline]
        #[target_feature(enable = "sse2")]
        v_escape::escape_ranges_bytes!(impl v_escape::loop_range_switch_sse2 where $($t)+);
    };
    (impl $loops:path where ($T:ident, $Q:ident, $Q_LEN:ident) $($t:tt)+) => {
        pub unsafe fn b_escape<B: v_escape::Buffer>(bytes: &[u8], buf: &mut B) {
            let len = bytes.len();
            let start_ptr = bytes.as_ptr();
            let end_ptr = bytes[len..].as_ptr();
            let mut ptr = start_ptr;

            let mut start = 0;

            macro_rules! mask_bodies_callback {
                ($callback:path) => {
                    // Format bytes in the mask that starts in the current pointer
                    macro_rules! mask_bodies {
                        ($mask:ident, $at:ident, $cur:ident, $ptr:ident) => {
                            // Calls macro `bodies!` at position `$at + $cur`
                            // of byte `*$ptr` + `$curr` with macro `v_escape::mask_body!`
                            $callback!($T, $Q, $Q_LEN, $at + $cur, *$ptr.add($cur), start, bytes, buf, v_escape::mask_body_bytes);

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

            v_escape::mask_bodies_escaping_bytes!($($t)+);

            // Macro to write with mask
            macro_rules! write_mask {
                ($mask:ident, $ptr:ident) => {{
                    // Reference to the start of mask
                    let at = v_escape::sub!($ptr, start_ptr);
                    // Get to the first possible escape character avoiding zeros
                    let mut cur = $mask.trailing_zeros() as usize;

                    loop {
                        // Writing in `$fmt` with `$mask`
                        // The main loop will break when mask == 0
                        mask_bodies!($mask, at, cur, $ptr);
                    }

                    debug_assert_eq!(at, v_escape::sub!($ptr, start_ptr))
                }};
            }

            // Write a sliced mask
            macro_rules! write_forward {
                ($mask: ident, $align:ident) => {{
                    let at = v_escape::sub!(ptr, start_ptr);
                    let mut cur = $mask.trailing_zeros() as usize;

                    while cur < $align {
                        mask_bodies!($mask, at, cur, ptr);
                    }

                    debug_assert_eq!(at, v_escape::sub!(ptr, start_ptr))
                }};
            }

            macro_rules! fallback_callback {
                (default) => {
                    macro_rules! fallback {
                        () => {
                            while ptr < end_ptr {
                                v_escape::bodies_bytes!(
                                    $T,
                                    $Q,
                                    $Q_LEN,
                                    v_escape::sub!(ptr, start_ptr),
                                    *ptr,
                                    start,
                                    bytes,
                                    buf,
                                    v_escape::mask_body_bytes
                                );
                                ptr = ptr.offset(1);
                            }
                        };
                    }
                };
                (one) => {
                    macro_rules! fallback {
                        () => {
                            while ptr < end_ptr {
                                if *ptr == $T {
                                    v_escape::bodies_exact_one_bytes!(
                                        $T,
                                        $Q,
                                        $Q_LEN,
                                        v_escape::sub!(ptr, start_ptr),
                                        *ptr,
                                        start,
                                        bytes,
                                        buf,
                                        v_escape::mask_body_bytes
                                    );
                                }
                                ptr = ptr.offset(1);
                            }
                        };
                    }
                };
            }

            v_escape::fallback_escaping!($($t)+);

            $loops!((len, ptr, start_ptr, end_ptr) $($t)+);

            // Write since start to the end of the slice
            debug_assert!(start <= len);
            if start < len {
                v_escape::write_bytes!(&bytes[start..], buf);
            }
        }
    };
}
