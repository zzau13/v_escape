use super::ArgLoop;
use crate::ranges::{Fallback, WriteMask};
use crate::utils::ident;
use proc_macro2::TokenStream;
use quote::quote;

pub fn loop_sse<WM: WriteMask, WF: WriteMask, F: Fallback>(
    ArgLoop {
        s,
        len,
        end_ptr,
        start_ptr,
        ptr,
        write_mask,
        write_forward,
        fallback,
    }: ArgLoop<WM, WF, F>,
) -> TokenStream {
    let (ref translations, masking) = s.translations_sse();
    let a = &ident("a");
    let masking_a = &masking(a);
    let mask = &ident("mask");
    let masked = &write_mask(mask, ptr);
    let align = &ident("align");
    let masked_forward = &write_forward(mask, align);
    let fall = fallback();

    quote! {
        const M128_VECTOR_SIZE: usize = std::mem::size_of::<__m128i>();
        const M128_VECTOR_ALIGN: usize = M128_VECTOR_SIZE - 1;

        if #len < M128_VECTOR_SIZE {
            #fall
        } else {
            #translations
            {
                let #align = M128_VECTOR_SIZE - (#start_ptr as usize & M128_VECTOR_ALIGN);
                if #align < M128_VECTOR_SIZE {
                    let mut #mask = {
                        let #a = _mm_loadu_si128(#ptr as *const __m128i);
                        _mm_movemask_epi8(#masking_a)
                    };
                    if #mask != 0 {
                        #masked_forward
                    }
                    #ptr = #ptr.add(#align);
                }
            }

            while #ptr <= #end_ptr.sub(M128_VECTOR_SIZE) {
                debug_assert_eq!(0, (#ptr as usize) % M128_VECTOR_SIZE);
                let mut #mask = {
                    let #a = _mm_load_si128(#ptr as *const __m128i);
                    _mm_movemask_epi8(#masking_a)
                };

                if #mask != 0 {
                    #masked;
                }
                #ptr = #ptr.add(M128_VECTOR_SIZE);
            }

            debug_assert!(#end_ptr.sub(M128_VECTOR_SIZE) < #ptr);

            if #ptr < #end_ptr {
                let d = M128_VECTOR_SIZE - sub(#end_ptr, #ptr);

                let mut #mask = ({
                    debug_assert_eq!(M128_VECTOR_SIZE, sub(#end_ptr, #ptr.sub(d)));
                    let #a = _mm_loadu_si128(#ptr.sub(d) as *const __m128i);
                    _mm_movemask_epi8(#masking_a)
                } as u16).wrapping_shr(d as u32);

                if #mask != 0 {
                    #masked
                }
            }
        }
    }
}
