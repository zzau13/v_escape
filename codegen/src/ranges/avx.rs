use proc_macro2::TokenStream;
use quote::quote;

use super::ArgLoop;

pub fn loop_range_switch_avx2(
    ArgLoop {
        s,
        len,
        end_ptr,
        start_ptr,
        ptr,
    }: ArgLoop,
) -> TokenStream {
    let sse = quote! {};
    let translations = s.translations_256();
    quote! {
        use std::arch::x86_64::{
            __m256i, _mm256_load_si256, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_or_si256,
        };

        const M256_VECTOR_SIZE: usize = std::mem::size_of::<__m256i>();
        const LOOP_SIZE: usize = 4 * M256_VECTOR_SIZE;

        if #len < M256_VECTOR_SIZE {
            #sse
        } else {
            #translations
            {
                const M256_VECTOR_ALIGN: usize = M256_VECTOR_SIZE - 1;
                let align = M256_VECTOR_SIZE - (#start_ptr as usize & M256_VECTOR_ALIGN);
                if align < M256_VECTOR_SIZE {
                    let mut mask = {
                        let a = _mm256_loadu_si256(#ptr as *const __m256i);
                        _mm256_movemask_epi8(masking!(a))
                    };

                    if mask != 0 {
                        write_forward!(mask, align);
                    }
                    #ptr = #ptr.add(align);
                }
            }

            if LOOP_SIZE <= #len {
                while #ptr <= #end_ptr.sub(LOOP_SIZE) {
                    debug_assert_eq!(0, (#ptr as usize) % _ONSWITCH_M256_VECTOR_SIZE);
                    let cmp_a = {
                        let a = _mm256_load_si256(#ptr as *const __m256i);
                        masking!(a)
                    };

                    let cmp_b = {
                        let a = _mm256_load_si256(#ptr.add(_ONSWITCH_M256_VECTOR_SIZE) as *const __m256i);
                        masking!(a)
                    };

                    let cmp_c = {
                        let a = _mm256_load_si256(#ptr.add(_ONSWITCH_M256_VECTOR_SIZE * 2) as *const __m256i);
                        masking!(a)
                    };

                    let cmp_d = {
                        let a = _mm256_load_si256(#ptr.add(_ONSWITCH_M256_VECTOR_SIZE * 3) as *const __m256i);
                        masking!(a)
                    };

                    if _mm256_movemask_epi8(_mm256_or_si256(
                        _mm256_or_si256(cmp_a, cmp_b),
                        _mm256_or_si256(cmp_c, cmp_d),
                    )) != 0
                    {
                        let mut mask = _mm256_movemask_epi8(cmp_a);
                        if mask != 0 {
                            write_mask!(mask, #ptr);
                        }

                        mask = _mm256_movemask_epi8(cmp_b);
                        if mask != 0 {
                            let #ptr = #ptr.add(_ONSWITCH_M256_VECTOR_SIZE);
                            write_mask!(mask, #ptr);
                        }

                        mask = _mm256_movemask_epi8(cmp_c);
                        if mask != 0 {
                            let #ptr = #ptr.add(_ONSWITCH_M256_VECTOR_SIZE * 2);
                            write_mask!(mask, #ptr);
                        }

                        mask = _mm256_movemask_epi8(cmp_d);
                        if mask != 0 {
                            let #ptr = #ptr.add(_ONSWITCH_M256_VECTOR_SIZE * 3);
                            write_mask!(mask, #ptr);
                        }
                    }

                    #ptr = #ptr.add(LOOP_SIZE);
                }
            }

            while #ptr <= #end_ptr.sub(M256_VECTOR_SIZE) {
                debug_assert_eq!(0, (#ptr as usize) % M256_VECTOR_SIZE);

                let mut mask = {
                    let a = _mm256_load_si256(#ptr as *const __m256i);
                    _mm256_movemask_epi8(masking!(a))
                };

                if mask != 0 {
                    write_mask!(mask, #ptr);
                }
                #ptr = #ptr.add(M256_VECTOR_SIZE);
            }

            debug_assert!(#end_ptr.sub(M256_VECTOR_SIZE) < #ptr);

            if #ptr < #end_ptr {
                let d = M256_VECTOR_SIZE - $crate::sub!(#end_ptr, #ptr);

                let mut mask = ({
                    debug_assert_eq!(M256_VECTOR_SIZE, $crate::sub!(#end_ptr, #ptr.sub(d)), "Over runs");
                    let a = _mm256_loadu_si256(#ptr.sub(d) as *const __m256i);
                    _mm256_movemask_epi8(masking!(a))
                } as u32).wrapping_shr(d as u32);

                if mask != 0 {
                    write_mask!(mask, #ptr);
                }
            }
        }
    }
}
