use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Switch {
    A {
        a: u8,
    },
    Ar {
        la: u8,
        ra: u8,
    },
    AB {
        a: u8,
        b: u8,
    },
    ArB {
        la: u8,
        ra: u8,
        b: u8,
    },
    ArBr {
        la: u8,
        ra: u8,
        lb: u8,
        rb: u8,
    },
    ABC {
        a: u8,
        b: u8,
        c: u8,
    },
    ArBC {
        la: u8,
        ra: u8,
        b: u8,
        c: u8,
    },
    ArBrC {
        la: u8,
        ra: u8,
        lb: u8,
        rb: u8,
        c: u8,
    },
    ArBrCr {
        la: u8,
        ra: u8,
        lb: u8,
        rb: u8,
        lc: u8,
        rc: u8,
    },
}

use Switch::*;

impl Switch {
    pub fn translations_256(&self) -> TokenStream {
        match *self {
            ArBC { la, ra, b, c } => quote! {
                use std::arch::x86_64::{
                    _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_or_si256,
                    _mm256_set1_epi8,
                };
                const TRANSLATION_A: i8 = i8::MAX - #ra;
                const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                const B: i8 = #b;
                const C: i8 = #c;

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
            },
            ABC { a, b, c } => quote! {
                use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_or_si256, _mm256_set1_epi8};
                const A: i8 = #a;
                const B: i8 = #b;
                const C: i8 = #c;

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
            },
            AB { a, b } => quote! {
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
            },
            A { a } => quote! {
                use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_set1_epi8};
                const A: i8 = $fa;

                let v_a = _mm256_set1_epi8(A);

                macro_rules! masking {
                    ($a:ident) => {{
                        _mm256_cmpeq_epi8($a, v_a)
                    }};
                }
            },
            ArBrCr {
                la,
                ra,
                lb,
                rb,
                lc,
                rc,
            } => quote! {
                use std::arch::x86_64::{
                    _mm256_add_epi8, _mm256_cmpgt_epi8, _mm256_or_si256, _mm256_set1_epi8,
                };
                const TRANSLATION_A: i8 = i8::MAX - #ra;
                const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                const TRANSLATION_B: i8 = i8::MAX - #rb;
                const BELOW_B: i8 = i8::MAX - (#rb - #lb) - 1;
                const TRANSLATION_C: i8 = i8::MAX - #rc;
                const BELOW_C: i8 = i8::MAX - (#rc - #lc) - 1;

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
            },
            ArBrC { la, ra, lb, rb, c } => quote! {
                use std::arch::x86_64::{
                    _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_or_si256,
                    _mm256_set1_epi8,
                };
                const TRANSLATION_A: i8 = i8::MAX - #ra;
                const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                const TRANSLATION_B: i8 = i8::MAX - #rb;
                const BELOW_B: i8 = i8::MAX - (#rb - #lb) - 1;
                const C: i8 = #c;

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
            },
            ArBr { la, ra, lb, rb } => quote! {
                use std::arch::x86_64::{
                    _mm256_add_epi8, _mm256_cmpgt_epi8, _mm256_or_si256, _mm256_set1_epi8,
                };
                const TRANSLATION_A: i8 = i8::MAX - #ra;
                const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                const TRANSLATION_B: i8 = i8::MAX - #rb;
                const BELOW_B: i8 = i8::MAX - (#rb - #lb) - 1;

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
            },
            ArB { la, ra, b } => quote! {

                use std::arch::x86_64::{
                    _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_or_si256,
                    _mm256_set1_epi8,
                };
                const TRANSLATION_A: i8 = i8::MAX - #ra;
                const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                const B: i8 = #b;

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
            },
            Ar { la, ra } => quote! {
                use std::arch::x86_64::{_mm256_add_epi8, _mm256_cmpgt_epi8, _mm256_set1_epi8};
                const TRANSLATION_A: i8 = i8::MAX - #ra;
                const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;

                let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
                let v_below_a = _mm256_set1_epi8(BELOW_A);

                macro_rules! masking {
                    ($a:expr) => {{
                        _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a)
                    }};
                }
            },
        }
    }

    pub fn translations_128(&self) -> TokenStream {
        match *self {
            ArBC { la, ra, b, c } => quote! {
                use std::arch::x86_64::{
                    _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8,
                };
                const TRANSLATION_A: i8 = i8::MAX - #ra;
                const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                const B: i8 = #b;
                const C: i8 = #c;

                let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
                let v_below_a = _mm_set1_epi8(BELOW_A);
                let v_b = _mm_set1_epi8(B);
                let v_c = _mm_set1_epi8(C);

                macro_rules! masking {
                    ($a:expr) => {{
                        _mm_or_si128(
                            _mm_or_si128(_mm_cmpeq_epi8($a, v_b), _mm_cmpeq_epi8($a, v_c)),
                            _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                        )
                    }};
                }
            },
            ABC { a, b, c } => quote! {
            use std::arch::x86_64::{_mm_cmpeq_epi8, _mm_or_si128, _mm_set1_epi8};
            const A: i8 = #a;
            const B: i8 = #b;
            const C: i8 = #c;

            let v_a = _mm_set1_epi8(A);
            let v_b = _mm_set1_epi8(B);
            let v_c = _mm_set1_epi8(C);

            macro_rules! masking {
                ($a:ident) => {{
                    _mm_or_si128(
                        _mm_or_si128(_mm_cmpeq_epi8($a, v_a), _mm_cmpeq_epi8($a, v_b)),
                        _mm_cmpeq_epi8($a, v_c),
                    )
                }};
            }
                },
            AB { a, b } => quote! {
            use std::arch::x86_64::{_mm_cmpeq_epi8, _mm_or_si128, _mm_set1_epi8};
            const A: i8 = #a;
            const B: i8 = #b;

            let v_a = _mm_set1_epi8(A);
            let v_b = _mm_set1_epi8(B);

            macro_rules! masking {
                ($a:ident) => {{
                    _mm_or_si128(_mm_cmpeq_epi8($a, v_a), _mm_cmpeq_epi8($a, v_b))
                }};
            }
                },
            A { a } => quote! {
            use std::arch::x86_64::{_mm_cmpeq_epi8, _mm_set1_epi8};
            const A: i8 = #a;

            let v_a = _mm_set1_epi8(A);

            macro_rules! masking {
                ($a:ident) => {{
                    _mm_cmpeq_epi8($a, v_a)
                }};
            }
                },
            ArBrCr {
                la,
                ra,
                lb,
                rb,
                lc,
                rc,
            } => quote! {
            use std::arch::x86_64::{_mm_add_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8};
            const TRANSLATION_A: i8 = i8::MAX - #ra;
            const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
            const TRANSLATION_B: i8 = i8::MAX - #rb;
            const BELOW_B: i8 = i8::MAX - (#rb - #lb) - 1;
            const TRANSLATION_C: i8 = i8::MAX - #rc;
            const BELOW_C: i8 = i8::MAX - (#rc - #lc) - 1;

            let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
            let v_below_a = _mm_set1_epi8(BELOW_A);
            let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
            let v_below_b = _mm_set1_epi8(BELOW_B);
            let v_translation_c = _mm_set1_epi8(TRANSLATION_C);
            let v_below_c = _mm_set1_epi8(BELOW_C);

            macro_rules! masking {
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
            },
            ArBrC { la, ra, lb, rb, c } => quote! {
            use std::arch::x86_64::{
                _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8,
            };
            const TRANSLATION_A: i8 = i8::MAX - #ra;
            const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
            const TRANSLATION_B: i8 = i8::MAX - #rb;
            const BELOW_B: i8 = i8::MAX - (#rb - #lb) - 1;
            const C: i8 = #c;

            let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
            let v_below_a = _mm_set1_epi8(BELOW_A);
            let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
            let v_below_b = _mm_set1_epi8(BELOW_B);
            let v_c = _mm_set1_epi8(C);

            macro_rules! masking {
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
                },
            ArBr { la, ra, lb, rb } => quote! {
            use std::arch::x86_64::{_mm_add_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8};
            const TRANSLATION_A: i8 = i8::MAX - #ra;
            const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
            const TRANSLATION_B: i8 = i8::MAX - #rb;
            const BELOW_B: i8 = i8::MAX - (#rb - #lb) - 1;

            let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
            let v_below_a = _mm_set1_epi8(BELOW_A);
            let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
            let v_below_b = _mm_set1_epi8(BELOW_B);

            macro_rules! masking {
                ($a:expr) => {{
                    _mm_or_si128(
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_b), v_below_b),
                    )
                }};
            }
                },
            ArB { la, ra, b } => quote! {
            use std::arch::x86_64::{
                _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8,
            };
            const TRANSLATION_A: i8 = i8::MAX - #ra;
            const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
            const B: i8 = #b;

            let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
            let v_below_a = _mm_set1_epi8(BELOW_A);
            let v_b = _mm_set1_epi8(B);

            macro_rules! masking {
                ($a:expr) => {{
                    _mm_or_si128(
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                        _mm_cmpeq_epi8($a, v_b),
                    )
                }};
            }
                },
            Ar { la, ra } => quote! {
            use std::arch::x86_64::{_mm_add_epi8, _mm_cmpgt_epi8, _mm_set1_epi8};
            const TRANSLATION_A: i8 = i8::MAX - #ra;
            const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;

            let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
            let v_below_a = _mm_set1_epi8(BELOW_A);

            macro_rules! masking {
                ($a:expr) => {{
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a)
                }};
            }
            },
        }
    }

    pub fn fallback_escaping(&self) -> TokenStream {
        match *self {
            A { .. } => quote! {
                fallback_callback!(one);
            },
            _ => quote! {
                fallback_callback!(default);
            },
        }
    }
}

#[macro_export]
#[doc(hidden)]
/// Generate mask bodies callback
///
/// Defining exact match or false positive
/// ## The following macros must be defined
///
/// * `mask_bodies_callback($callback:path)`
///     select between `mask_bodies`
///
macro_rules! mask_bodies_escaping {
    ($fa:expr, 128, ) => {
        mask_bodies_callback!($crate::bodies_exact_one);
    };
    ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
        mask_bodies_callback!($crate::bodies);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
        mask_bodies_callback!($crate::bodies);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
        mask_bodies_callback!($crate::bodies);
    };
    ($($t:tt)+) => {
        mask_bodies_callback!($crate::bodies_exact);
    };
}

#[macro_export]
#[doc(hidden)]
/// Generate mask bodies callback
///
/// Defining exact match or false positive
/// ## The following macros must be defined
///
/// * `mask_bodies_callback($callback:path)`
///     select between `mask_bodies`
///
macro_rules! mask_bodies_escaping_ptr {
    ($fa:expr, 128, ) => {
        mask_bodies_callback!($crate::bodies_exact_one_ptr);
    };
    ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
        mask_bodies_callback!($crate::bodies_ptr);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
        mask_bodies_callback!($crate::bodies_ptr);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
        mask_bodies_callback!($crate::bodies_ptr);
    };
    ($($t:tt)+) => {
        mask_bodies_callback!($crate::bodies_exact_ptr);
    };
}

#[macro_export]
#[doc(hidden)]
/// Generate mask bodies callback
///
/// Defining exact match or false positive
/// ## The following macros must be defined
///
/// * `mask_bodies_callback($callback:path)`
///     select between `mask_bodies`
///
macro_rules! mask_bodies_escaping_bytes {
    ($fa:expr, 128, ) => {
        mask_bodies_callback!($crate::bodies_exact_one_bytes);
    };
    ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
        mask_bodies_callback!($crate::bodies_bytes);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
        mask_bodies_callback!($crate::bodies_bytes);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
        mask_bodies_callback!($crate::bodies_bytes);
    };
    ($($t:tt)+) => {
        mask_bodies_callback!($crate::bodies_exact_bytes);
    };
}

#[macro_export]
#[doc(hidden)]
/// Generate mask bodies callback
///
/// Defining exact match or false positive
///
/// ## The following macros must be defined
///
///  * `$crate::switch_main_loop!(impl [024] for ($len:ident, $ptr:ident, $end_ptr:ident)`
///    switch between main loops in avx
///
macro_rules! avx_main_loop {
    (($len:ident, $ptr:ident, $end_ptr:ident) $($t:tt, )+) => {
        macro_rules! _inside {
            ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
                $crate::switch_main_loop!(impl 4 for ($len, $ptr, $end_ptr));};
            ($fa:expr, $fb:expr, $fc:expr, 128, ) => {
                $crate::switch_main_loop!(impl 4 for ($len, $ptr, $end_ptr));};
            ($fa:expr, $fb:expr, 128, ) => {
                $crate::switch_main_loop!(impl 4 for ($len, $ptr, $end_ptr));};
            ($fa:expr, 128, ) => {
                $crate::switch_main_loop!(impl 4 for ($len, $ptr, $end_ptr));};
            // TODO: https://github.com/rust-lang-nursery/stdsimd/issues/674
            ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
                $crate::switch_main_loop!(impl 4 for ($len, $ptr, $end_ptr));};
            ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
                $crate::switch_main_loop!(impl 4 for ($len, $ptr, $end_ptr));};
            ($la:expr, $ra:expr, $lb:expr, $rb:expr, ) => {
                $crate::switch_main_loop!(impl 4 for ($len, $ptr, $end_ptr));};
            ($la:expr, $ra:expr, $b:expr, ) => {
                $crate::switch_main_loop!(impl 4 for ($len, $ptr, $end_ptr));
            };
            ($la:expr, $ra:expr, ) => {
                $crate::switch_main_loop!(impl 4 for ($len, $ptr, $end_ptr));
            };
        }

        _inside!($($t, )+);
    };
}

#[macro_export]
#[doc(hidden)]
/// Generate avx main loops
macro_rules! switch_main_loop {
    (impl 0 for ($len:ident, $ptr:ident, $end_ptr:ident)) => {
    };
    (impl 2 for ($len:ident, $ptr:ident, $end_ptr:ident)) => {
        const _ONSWITCH_M256_VECTOR_SIZE: usize = std::mem::size_of::<__m256i>();
        const LOOP_SIZE: usize = 2 * _ONSWITCH_M256_VECTOR_SIZE;

        if LOOP_SIZE <= $len {
            while $ptr <= $end_ptr.sub(LOOP_SIZE) {
                debug_assert_eq!(0, ($ptr as usize) % _ONSWITCH_M256_VECTOR_SIZE);

                // Using function `_mm256_load_si256` for faster behavior on aligned bytes.
                // Getting 2sets of $length `_ONSWITCH_M256_VECTOR_SIZE` each (`LOOP_SIZE=4*_ONSWITCH_M256_VECTOR_SIZE`)
                let cmp_a = {
                    let a = _mm256_load_si256($ptr as *const __m256i);
                    masking!(a)
                };

                let cmp_b = {
                    let a = _mm256_load_si256($ptr.add(_ONSWITCH_M256_VECTOR_SIZE) as *const __m256i);
                    masking!(a)
                };

                // Combining the 2 sets using `or` logic operator by pairs. Then we write the
                // mask for the 2 sets of `_ONSWITCH_M256_VECTOR_SIZE` elements each, and make the pointer
                // point to the next `LOOP_SIZE` elements
                if _mm256_movemask_epi8(_mm256_or_si256(cmp_a, cmp_b)) != 0
                {
                    let mut mask = _mm256_movemask_epi8(cmp_a);
                    if mask != 0 {
                        write_mask!(mask, $ptr);
                    }

                    mask = _mm256_movemask_epi8(cmp_b);
                    if mask != 0 {
                        let $ptr = $ptr.add(_ONSWITCH_M256_VECTOR_SIZE);
                        write_mask!(mask, $ptr);
                    }

                }

                $ptr = $ptr.add(LOOP_SIZE);
            }
        }
    };
    (impl 4 for ($len:ident, $ptr:ident, $end_ptr:ident)) => {

    };
}
