use proc_macro2::TokenStream;
use quote::quote;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
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

enum Bodies {
    Reg,
    Exact,
}

#[allow(clippy::from_over_into)]
impl Into<Bodies> for Switch {
    fn into(self) -> Bodies {
        use Switch::*;
        match self {
            ArBC { .. } | ArBrCr { .. } | ArBrC { .. } => Bodies::Reg,
            _ => Bodies::Exact,
        }
    }
}

pub(crate) struct Masking {
    pub struct_body: TokenStream,
    pub build: TokenStream,
    pub mask_body: TokenStream,
    pub false_positive: bool,
}

impl Masking {
    fn new(
        struct_body: TokenStream,
        build: TokenStream,
        mask_body: TokenStream,
        false_positive: bool,
    ) -> Self {
        Self {
            struct_body,
            build,
            mask_body,
            false_positive,
        }
    }
}

impl From<Switch> for Masking {
    fn from(switch: Switch) -> Self {
        use Switch::*;

        match switch {
            ArBC { la, ra, b, c } => {
                let translation_a: i8 = i8::MAX - ra;
                let below_a: i8 = i8::MAX - (ra - la) - 1;
                Masking::new(
                    quote! {{
                        translation_a: V,
                        below_a: V,
                        b: V,
                        c: V
                    }},
                    quote! {
                        Self::Escapes {
                            translation_a: V::splat(#translation_a as u8),
                            below_a: V::splat(#below_a as u8),
                            b: V::splat(#b as u8),
                            c: V::splat(#c as u8),
                        }
                    },
                    quote! {
                        vector2.add(self.translation_a).gt(self.below_a)
                            .or(vector2.cmpeq(self.b))
                            .or(vector2.cmpeq(self.c))
                    },
                    false,
                )
            }
            ABC { a, b, c } => Masking::new(
                quote! {{
                    a: V,
                    b: V,
                    c: V
                }},
                quote! {
                    Self::Escapes {
                        a: V::splat(#a as u8),
                        b: V::splat(#b as u8),
                        c: V::splat(#c as u8),
                    }
                },
                quote! {
                        vector2.cmpeq(self.a)
                            .or(vector2.cmpeq(self.b))
                            .or(vector2.cmpeq(self.c))
                },
                false,
            ),
            AB { a, b } => Masking::new(
                quote! {{
                    a: V,
                    b: V
                }},
                quote! {
                    Self::Escapes {
                        a: V::splat(#a as u8),
                        b: V::splat(#b as u8),
                    }
                },
                quote! {
                        vector2.cmpeq(self.a)
                            .or(vector2.cmpeq(self.b))
                },
                false,
            ),
            A { a } => Masking::new(
                quote! {{
                    a: V
                }},
                quote! {
                    Self::Escapes {
                        a: V::splat(#a as u8),
                    }
                },
                quote! {
                    vector2.cmpeq(self.a)
                },
                false,
            ),
            ArBrCr {
                la,
                ra,
                lb,
                rb,
                lc,
                rc,
            } => {
                let translation_a: i8 = i8::MAX - ra;
                let below_a: i8 = i8::MAX - (ra - la) - 1;
                let translation_b: i8 = i8::MAX - rb;
                let below_b: i8 = i8::MAX - (rb - lb) - 1;
                let translation_c: i8 = i8::MAX - rc;
                let below_c: i8 = i8::MAX - (rc - lc) - 1;
                Masking::new(
                    quote! {{
                        translation_a: V,
                        below_a: V,
                        translation_b: V,
                        below_b: V,
                        translation_c: V,
                        below_c: V
                    }},
                    quote! {
                        Self::Escapes {
                            translation_a: V::splat(#translation_a as u8),
                            below_a: V::splat(#below_a as u8),
                            translation_b: V::splat(#translation_b as u8),
                            below_b: V::splat(#below_b as u8),
                            translation_c: V::splat(#translation_c as u8),
                            below_c: V::splat(#below_c as u8),
                        }
                    },
                    quote! {
                        vector2.add(self.translation_a).gt(self.below_a)
                            .or(vector2.add(self.translation_b).gt(self.below_b))
                            .or(vector2.add(self.translation_c).gt(self.below_c))
                    },
                    true,
                )
            }
            ArBrC { la, ra, lb, rb, c } => {
                let translation_a: i8 = i8::MAX - ra;
                let below_a: i8 = i8::MAX - (ra - la) - 1;
                let translation_b: i8 = i8::MAX - rb;
                let below_b: i8 = i8::MAX - (rb - lb) - 1;
                Masking::new(
                    quote! {{
                        translation_a: V,
                        below_a: V,
                        translation_b: V,
                        below_b: V,
                        c: V
                    }},
                    quote! {
                        Self::Escapes {
                            translation_a: V::splat(#translation_a as u8),
                            below_a: V::splat(#below_a as u8),
                            translation_b: V::splat(#translation_b as u8),
                            below_b: V::splat(#below_b as u8),
                            c: V::splat(#c as u8),
                        }
                    },
                    quote! {
                        vector2.add(self.translation_a).gt(self.below_a)
                            .or(vector2.add(self.translation_b).gt(self.below_b))
                            .or(vector2.cmpeq(self.c))
                    },
                    true,
                )
            }
            ArBr { la, ra, lb, rb } => {
                let translation_a: i8 = i8::MAX - ra;
                let below_a: i8 = i8::MAX - (ra - la) - 1;
                let translation_b: i8 = i8::MAX - rb;
                let below_b: i8 = i8::MAX - (rb - lb) - 1;
                Masking::new(
                    quote! {{
                        translation_a: V,
                        below_a: V,
                        translation_b: V,
                        below_b: V
                    }},
                    quote! {
                        Self::Escapes {
                            translation_a: V::splat(#translation_a as u8),
                            below_a: V::splat(#below_a as u8),
                            translation_b: V::splat(#translation_b as u8),
                            below_b: V::splat(#below_b as u8),
                        }
                    },
                    quote! {
                        vector2.add(self.translation_a).gt(self.below_a)
                            .or(vector2.add(self.translation_b).gt(self.below_b))
                    },
                    false,
                )
            }
            ArB { la, ra, b } => {
                let translation_a: i8 = i8::MAX - ra;
                let below_a: i8 = i8::MAX - (ra - la) - 1;
                Masking::new(
                    quote! {{
                        translation_a: V,
                        below_a: V,
                        b: V
                    }},
                    quote! {
                        Self::Escapes {
                            translation_a: V::splat(#translation_a as u8),
                            below_a: V::splat(#below_a as u8),
                            b: V::splat(#b as u8),
                        }
                    },
                    quote! {
                        vector2.add(self.translation_a).gt(self.below_a)
                            .or(vector2.cmpeq(self.b))
                    },
                    false,
                )
            }
            Ar { la, ra } => {
                let translation_a: i8 = i8::MAX - ra;
                let below_a: i8 = i8::MAX - (ra - la) - 1;
                Masking::new(
                    quote! {{
                        translation_a: V,
                        below_a: V
                    }},
                    quote! {
                        Self::Escapes {
                            translation_a: V::splat(#translation_a as u8),
                            below_a: V::splat(#below_a as u8),
                        }
                    },
                    quote! {
                        vector2.add(self.translation_a).gt(self.below_a)
                    },
                    false,
                )
            }
        }
    }
}
