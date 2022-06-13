use proc_macro2::TokenStream;
use quote::quote;

use crate::ranges::{sse, WriteMask};
use crate::utils::ident;

use super::ArgLoop;

pub fn loop_avx2<F: WriteMask>(arg: ArgLoop<F>) -> TokenStream {
    let sse = sse::loop_range_switch_sse(arg);
    let ArgLoop {
        s,
        len,
        end_ptr,
        start_ptr,
        ptr,
        write_mask,
    } = arg;
    let a = &ident("a");
    let (ref translations, masking) = s.translations_avx();
    let masking_a = &masking(a);
    let mask = &ident("mask");
    let masked = &write_mask(mask, ptr);

    quote! {
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
                    let mut #mask = {
                        let #a = _mm256_loadu_si256(#ptr as *const __m256i);
                        _mm256_movemask_epi8(#masking_a)
                    };

                    if #mask != 0 {
                        write_forward!(mask, align);
                    }
                    #ptr = #ptr.add(align);
                }
            }

            if LOOP_SIZE <= #len {
                while #ptr <= #end_ptr.sub(LOOP_SIZE) {
                    debug_assert_eq!(0, (#ptr as usize) % M256_VECTOR_SIZE);
                    let cmp_a = {
                        let #a = _mm256_load_si256(#ptr as *const __m256i);
                        #masking_a
                    };

                    let cmp_b = {
                        let #a = _mm256_load_si256(#ptr.add(M256_VECTOR_SIZE) as *const __m256i);
                        #masking_a
                    };

                    let cmp_c = {
                        let #a = _mm256_load_si256(#ptr.add(M256_VECTOR_SIZE * 2) as *const __m256i);
                        #masking_a
                    };

                    let cmp_d = {
                        let #a = _mm256_load_si256(#ptr.add(M256_VECTOR_SIZE * 3) as *const __m256i);
                        #masking_a
                    };

                    if _mm256_movemask_epi8(_mm256_or_si256(
                        _mm256_or_si256(cmp_a, cmp_b),
                        _mm256_or_si256(cmp_c, cmp_d),
                    )) != 0
                    {
                        let mut #mask = _mm256_movemask_epi8(cmp_a);
                        if #mask != 0 {
                            #masked
                        }

                        #mask = _mm256_movemask_epi8(cmp_b);
                        if #mask != 0 {
                            let #ptr = #ptr.add(M256_VECTOR_SIZE);
                            #masked
                        }

                        #mask = _mm256_movemask_epi8(cmp_c);
                        if #mask != 0 {
                            let #ptr = #ptr.add(M256_VECTOR_SIZE * 2);
                            #masked
                        }

                        #mask = _mm256_movemask_epi8(cmp_d);
                        if #mask != 0 {
                            let #ptr = #ptr.add(M256_VECTOR_SIZE * 3);
                            #masked
                        }
                    }

                    #ptr = #ptr.add(LOOP_SIZE);
                }
            }

            while #ptr <= #end_ptr.sub(M256_VECTOR_SIZE) {
                debug_assert_eq!(0, (#ptr as usize) % M256_VECTOR_SIZE);

                let mut #mask = {
                    let #a = _mm256_load_si256(#ptr as *const __m256i);
                    _mm256_movemask_epi8(#masking_a)
                };

                if #mask != 0 {
                    #masked
                }
                #ptr = #ptr.add(M256_VECTOR_SIZE);
            }

            debug_assert!(#end_ptr.sub(M256_VECTOR_SIZE) < #ptr);

            if #ptr < #end_ptr {
                let d = M256_VECTOR_SIZE - crate::sub!(#end_ptr, #ptr);

                let mut #mask = ({
                    debug_assert_eq!(M256_VECTOR_SIZE, crate::sub!(#end_ptr, #ptr.sub(d)), "Over runs");
                    let #a = _mm256_loadu_si256(#ptr.sub(d) as *const __m256i);
                    _mm256_movemask_epi8(#masking_a)
                } as u32).wrapping_shr(d as u32);

                if #mask != 0 {
                    #masked
                }
            }
        }
    }
}
