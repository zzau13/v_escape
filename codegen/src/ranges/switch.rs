use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::macros::{bodies, Bodies, BodiesArg, CB};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Switch {
    A {
        a: i8,
    },
    Ar {
        la: i8,
        ra: i8,
    },
    AB {
        a: i8,
        b: i8,
    },
    ArB {
        la: i8,
        ra: i8,
        b: i8,
    },
    ArBr {
        la: i8,
        ra: i8,
        lb: i8,
        rb: i8,
    },
    ABC {
        a: i8,
        b: i8,
        c: i8,
    },
    ArBC {
        la: i8,
        ra: i8,
        b: i8,
        c: i8,
    },
    ArBrC {
        la: i8,
        ra: i8,
        lb: i8,
        rb: i8,
        c: i8,
    },
    ArBrCr {
        la: i8,
        ra: i8,
        lb: i8,
        rb: i8,
        lc: i8,
        rc: i8,
    },
}

use Switch::*;

impl Into<Bodies> for Switch {
    fn into(self) -> Bodies {
        match self {
            ArBC { .. } | ArBrCr { .. } | ArBrC { .. } => Bodies::Reg,
            _ => Bodies::Exact,
        }
    }
}

impl Switch {
    pub fn translations_avx(&self) -> (TokenStream, fn(&Ident) -> TokenStream) {
        match *self {
            ArBC { la, ra, b, c } => (
                quote! {
                    const TRANSLATION_A: i8 = i8::MAX - #ra;
                    const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                    const B: i8 = #b;
                    const C: i8 = #c;

                    let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm256_set1_epi8(BELOW_A);
                    let v_b = _mm256_set1_epi8(B);
                    let v_c = _mm256_set1_epi8(C);
                },
                |a: &Ident| {
                    quote! {
                        _mm256_or_si256(
                            _mm256_or_si256(_mm256_cmpeq_epi8(#a, v_b), _mm256_cmpeq_epi8(#a, v_c)),
                            _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_a), v_below_a),
                        )
                    }
                },
            ),
            ABC { a, b, c } => (
                quote! {
                    const A: i8 = #a;
                    const B: i8 = #b;
                    const C: i8 = #c;

                    let v_a = _mm256_set1_epi8(A);
                    let v_b = _mm256_set1_epi8(B);
                    let v_c = _mm256_set1_epi8(C);
                },
                |a| {
                    quote! {
                         _mm256_or_si256(
                            _mm256_or_si256(_mm256_cmpeq_epi8(#a, v_a), _mm256_cmpeq_epi8(#a, v_b)),
                            _mm256_cmpeq_epi8(#a, v_c),
                        )
                    }
                },
            ),
            AB { a, b } => (
                quote! {
                    const A: i8 = #a;
                    const B: i8 = #b;

                    let v_a = _mm256_set1_epi8(A);
                    let v_b = _mm256_set1_epi8(B);
                },
                |a| {
                    quote! {
                        _mm256_or_si256(_mm256_cmpeq_epi8(#a, v_a), _mm256_cmpeq_epi8(#a, v_b))
                    }
                },
            ),
            A { a } => (
                quote! {
                    const A: i8 = #a;

                    let v_a = _mm256_set1_epi8(A);

                },
                |a| {
                    quote! {
                        _mm256_cmpeq_epi8(#a, v_a)
                    }
                },
            ),
            ArBrCr {
                la,
                ra,
                lb,
                rb,
                lc,
                rc,
            } => (
                quote! {
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
                },
                |a| {
                    quote! {
                        _mm256_or_si256(
                            _mm256_or_si256(
                                _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_a), v_below_a),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_b), v_below_b),
                            ),
                            _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_c), v_below_c),
                        )
                    }
                },
            ),
            ArBrC { la, ra, lb, rb, c } => (
                quote! {
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
                },
                |a| {
                    quote! {
                        _mm256_or_si256(
                            _mm256_or_si256(
                                _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_a), v_below_a),
                                _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_b), v_below_b),
                            ),
                            _mm256_cmpeq_epi8(#a, v_c),
                        )
                    }
                },
            ),
            ArBr { la, ra, lb, rb } => (
                quote! {
                    const TRANSLATION_A: i8 = i8::MAX - #ra;
                    const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                    const TRANSLATION_B: i8 = i8::MAX - #rb;
                    const BELOW_B: i8 = i8::MAX - (#rb - #lb) - 1;

                    let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm256_set1_epi8(BELOW_A);
                    let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
                    let v_below_b = _mm256_set1_epi8(BELOW_B);
                },
                |a| {
                    quote! {
                        _mm256_or_si256(
                            _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_a), v_below_a),
                            _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_b), v_below_b),
                        )
                    }
                },
            ),
            ArB { la, ra, b } => (
                quote! {
                    const TRANSLATION_A: i8 = i8::MAX - #ra;
                    const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                    const B: i8 = #b;

                    let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm256_set1_epi8(BELOW_A);
                    let v_b = _mm256_set1_epi8(B);
                },
                |a| {
                    quote! {
                        _mm256_or_si256(
                            _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_a), v_below_a),
                            _mm256_cmpeq_epi8(#a, v_b),
                        )
                    }
                },
            ),
            Ar { la, ra } => (
                quote! {
                    const TRANSLATION_A: i8 = i8::MAX - #ra;
                    const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;

                    let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm256_set1_epi8(BELOW_A);
                },
                |a| {
                    quote! {
                        _mm256_cmpgt_epi8(_mm256_add_epi8(#a, v_translation_a), v_below_a)
                    }
                },
            ),
        }
    }

    pub fn translations_sse(&self) -> (TokenStream, fn(&Ident) -> TokenStream) {
        match *self {
            ArBC { la, ra, b, c } => (
                quote! {
                    const TRANSLATION_A: i8 = i8::MAX - #ra;
                    const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                    const B: i8 = #b;
                    const C: i8 = #c;

                    let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm_set1_epi8(BELOW_A);
                    let v_b = _mm_set1_epi8(B);
                    let v_c = _mm_set1_epi8(C);
                },
                |a: &Ident| {
                    quote! {
                        _mm_or_si128(
                            _mm_or_si128(_mm_cmpeq_epi8(#a, v_b), _mm_cmpeq_epi8(#a, v_c)),
                            _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_a), v_below_a),
                        )
                    }
                },
            ),
            ABC { a, b, c } => (
                quote! {
                    const A: i8 = #a;
                    const B: i8 = #b;
                    const C: i8 = #c;

                    let v_a = _mm_set1_epi8(A);
                    let v_b = _mm_set1_epi8(B);
                    let v_c = _mm_set1_epi8(C);
                },
                |a| {
                    quote! {
                         _mm_or_si128(
                            _mm_or_si128(_mm_cmpeq_epi8(#a, v_a), _mm_cmpeq_epi8(#a, v_b)),
                            _mm_cmpeq_epi8(#a, v_c),
                        )
                    }
                },
            ),
            AB { a, b } => (
                quote! {
                    const A: i8 = #a;
                    const B: i8 = #b;

                    let v_a = _mm_set1_epi8(A);
                    let v_b = _mm_set1_epi8(B);
                },
                |a| {
                    quote! {
                        _mm_or_si128(_mm_cmpeq_epi8(#a, v_a), _mm_cmpeq_epi8(#a, v_b))
                    }
                },
            ),
            A { a } => (
                quote! {
                    const A: i8 = #a;

                    let v_a = _mm_set1_epi8(A);

                },
                |a| {
                    quote! {
                        _mm_cmpeq_epi8(#a, v_a)
                    }
                },
            ),
            ArBrCr {
                la,
                ra,
                lb,
                rb,
                lc,
                rc,
            } => (
                quote! {
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
                },
                |a| {
                    quote! {
                        _mm_or_si128(
                            _mm_or_si128(
                                _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_a), v_below_a),
                                _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_b), v_below_b),
                            ),
                            _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_c), v_below_c),
                        )
                    }
                },
            ),
            ArBrC { la, ra, lb, rb, c } => (
                quote! {
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
                },
                |a| {
                    quote! {
                        _mm_or_si128(
                            _mm_or_si128(
                                _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_a), v_below_a),
                                _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_b), v_below_b),
                            ),
                            _mm_cmpeq_epi8(#a, v_c),
                        )
                    }
                },
            ),
            ArBr { la, ra, lb, rb } => (
                quote! {
                    const TRANSLATION_A: i8 = i8::MAX - #ra;
                    const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                    const TRANSLATION_B: i8 = i8::MAX - #rb;
                    const BELOW_B: i8 = i8::MAX - (#rb - #lb) - 1;

                    let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm_set1_epi8(BELOW_A);
                    let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
                    let v_below_b = _mm_set1_epi8(BELOW_B);
                },
                |a| {
                    quote! {
                        _mm_or_si128(
                            _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_a), v_below_a),
                            _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_b), v_below_b),
                        )
                    }
                },
            ),
            ArB { la, ra, b } => (
                quote! {
                    const TRANSLATION_A: i8 = i8::MAX - #ra;
                    const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;
                    const B: i8 = #b;

                    let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm_set1_epi8(BELOW_A);
                    let v_b = _mm_set1_epi8(B);
                },
                |a| {
                    quote! {
                        _mm_or_si128(
                            _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_a), v_below_a),
                            _mm_cmpeq_epi8(#a, v_b),
                        )
                    }
                },
            ),
            Ar { la, ra } => (
                quote! {
                    const TRANSLATION_A: i8 = i8::MAX - #ra;
                    const BELOW_A: i8 = i8::MAX - (#ra - #la) - 1;

                    let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
                    let v_below_a = _mm_set1_epi8(BELOW_A);
                },
                |a| {
                    quote! {
                        _mm_cmpgt_epi8(_mm_add_epi8(#a, v_translation_a), v_below_a)
                    }
                },
            ),
        }
    }

    pub fn fallback_escaping<C: CB>(&self, b: BodiesArg<C>) -> TokenStream {
        bodies((*self).into(), b)
    }
}
