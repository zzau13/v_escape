use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};
use syn::{
    Ident, Lit, Token, parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    punctuated::Punctuated,
    token::{Not, Paren},
};

use crate::{
    pairs::{Pair, Pairs},
    switch::{self, Switch},
};

/// Parse template and return pairs
pub fn parse_template(tokens: TokenStream) -> syn::Result<Vec<Pair>> {
    let mut builder = syn::parse2::<Builder>(tokens)?;
    let mut pairs = builder.build();

    // need order for calculate ranges
    pairs.sort_by_key(|p| p.ch);
    // check repeated
    for i in 0..pairs.len() - 1 {
        let p1 = &pairs[i];
        let p2 = &pairs[i + 1];
        // TODO: to syn::Error
        assert_ne!(p1.ch, p2.ch, "{:?} and {:?} are repeated", p1, p2);
    }

    Ok(pairs)
}

struct Ch(i8);

impl Ch {
    fn new(n: u8) -> Result<Self, ()> {
        match n < (i8::MAX as u8) {
            true => Ok(Ch(n as i8)),
            false => Err(()),
        }
    }
}

impl Parse for Ch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit: &Lit = &input.parse()?;
        let map_err = |_: ()| syn::Error::new(lit.span(), "Only accept ASCII characters");
        Ok(match lit {
            Lit::Byte(v) => Ch::new(v.value()).map_err(map_err)?,
            Lit::Char(v) => Ch::new(v.value() as u8).map_err(map_err)?,
            Lit::Int(v) => v
                .base10_parse::<i8>()
                .map_err(|_| ())
                .and_then(|n| Ch::new(n as u8))
                .map_err(map_err)?,
            _ => return Err(map_err(())),
        })
    }
}

struct PairBuilder {
    ch: Ch,
    _s: Token![->],
    quote: syn::LitStr,
}

impl Parse for PairBuilder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(PairBuilder {
            ch: input.parse()?,
            _s: input.parse()?,
            quote: input.parse()?,
        })
    }
}

/// TODO: documentation
struct Builder {
    _path: Ident,
    _bang_token: Not,
    _delimiter: Paren,
    pairs: Punctuated<PairBuilder, Token![,]>,
    _dots: Token![;],
}

impl Parse for Builder {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> syn::Result<Self> {
        let tokens;
        Ok(Self {
            _path: input.parse()?,
            _bang_token: input.parse()?,
            _delimiter: parenthesized!(tokens in input),
            pairs: Punctuated::parse_separated_nonempty(&tokens)?,
            _dots: input.parse()?,
        })
    }
}

impl Builder {
    /// Consume and return arguments data
    fn build(self) -> Vec<Pair> {
        let Builder { pairs, .. } = self;

        pairs
            .into_pairs()
            .map(|x| x.into_value())
            .map(|x| Pair::new(x.ch.0, x.quote.value()))
            .collect()
    }
}

/// TODO: documentation
pub(crate) type Tables = (Ident, Ident, Ident);

/// TODO: documentation
pub(crate) struct Generator<'a> {
    pairs: &'a [Pair],
}

impl Generator<'_> {
    pub fn new(pairs: &[Pair]) -> Generator<'_> {
        Generator { pairs }
    }

    /// Builds a TokenStream containing the generated code for character escaping.
    ///
    /// This method creates a new TokenStream and writes the static lookup tables
    /// for character escaping into it. The tables include:
    /// - A mapping of characters to their escape sequences
    /// - The length of the escape sequences
    ///
    /// The generated code can be used to efficiently escape characters in strings.
    pub fn build(&self) -> TokenStream {
        let mut buf = TokenStream::new();

        let tables = self.write_static_table(&mut buf);
        self.write_impl(&mut buf);

        buf
    }

    fn write_static_table(&self, buf: &mut TokenStream) -> Tables {
        let len = self.pairs.len();
        let v_char = Ident::new("V_ESCAPE_CHARS", proc_macro2::Span::call_site());
        let v_quotes = Ident::new("V_ESCAPE_QUOTES", proc_macro2::Span::call_site());
        let v_len = Ident::new("V_ESCAPE_LEN", proc_macro2::Span::call_site());

        if len == 1 {
            let ch = self.pairs[0].ch;
            let quote = &self.pairs[0].quote;
            buf.extend(quote! {
                const #v_char: u8 = #ch;
                static #v_quotes: &str = #quote;
            })
        } else {
            let mut chs = Vec::with_capacity(256);
            let sch = self.pairs[0].ch as u8;
            for i in 0..=255_u8 {
                let n = self.pairs.binary_search_by(|s| s.ch.cmp(&i)).unwrap_or(len);
                chs.push(n as u8)
            }
            let quotes: Vec<&str> = self.pairs.iter().map(|s| s.quote.as_str()).collect();
            buf.extend(quote! {
                static #v_char: [u8; 256] = [#(#chs),*];
                static #v_quotes: [&str; #len] = [#(#quotes),*];
            })
        }
        buf.extend(quote! {
            const #v_len: usize = #len;
        });

        (v_char, v_quotes, v_len)
    }

    fn write_impl(&self, buf: &mut TokenStream) {
        let switch: Switch = Pairs(self.pairs).into();
        let (struct_body, build, mask_body) = switch.masking();
        let escape_len = self.pairs.len();
        // TODO: check false positive
        let false_positive = true;
        let q = quote! {
        use v_escape_base::{escape_builder, Escapes, EscapesBuilder, Vector};

        #[derive(Debug, Clone, Copy)]
        struct Escape<V: Vector> #struct_body

        struct Builder;
        impl EscapesBuilder for Builder {
            type Escapes<V: Vector> = Escape<V>;

            unsafe fn new<V: Vector>() -> Self::Escapes<V> {
                #build
            }
        }

        impl<V: Vector> Escapes for Escape<V> {
            const ESCAPE_LEN: usize = #escape_len;

            const FALSE_POSITIVE: bool = #false_positive;

            type Vector = V;

            #[inline(always)]
            unsafe fn masking(&self, vector2: V) -> V {
                #mask_body
            }

            #[inline(always)]
            fn escape(i: usize) -> &'static str {
                V_ESCAPE_QUOTES[i]
            }

            #[inline(always)]
            fn position(i: u8) -> usize {
                V_ESCAPE_CHARS[i as usize] as usize
            }

            #[inline(always)]
            fn byte_byte_compare(c: u8) -> bool {
                (V_ESCAPE_CHARS[c as usize] as usize) < V_ESCAPE_LEN
            }
        }

        escape_builder!(Builder);
        };
        buf.extend(q);
    }
}
