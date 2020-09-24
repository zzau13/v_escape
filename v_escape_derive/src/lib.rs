extern crate proc_macro;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::{
    parse::{Parse, ParseBuffer},
    spanned::Spanned,
    Token,
};

mod generator;
mod parser;

#[proc_macro]
pub fn derive(input: TokenStream) -> TokenStream {
    let (avx, pairs, print, simd) = match syn::parse::<StructBuilder>(input).and_then(|s| s.build())
    {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let code = generator::generate(&parser::parse(&pairs), simd, avx);

    if print {
        eprintln!("{}", code);
    }

    code.parse().unwrap()
}

type Struct = (bool, String, bool, bool);

struct MetaOpt<Lit: Parse> {
    pub path: syn::Path,
    pub eq_token: Token![=],
    pub lit: Lit,
}

impl<Lit: Parse> Parse for MetaOpt<Lit> {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            eq_token: input.parse()?,
            lit: input.parse()?,
        })
    }
}

struct StructBuilder {
    pub pairs: syn::LitStr,
    pub comma: Option<Token![,]>,
    pub opts: Punctuated<MetaOpt<syn::LitBool>, Token![,]>,
}

impl Parse for StructBuilder {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> syn::Result<Self> {
        Ok(Self {
            pairs: input.parse()?,
            comma: input.parse()?,
            opts: Punctuated::parse_terminated(input)?,
        })
    }
}

impl StructBuilder {
    fn build(self) -> syn::Result<Struct> {
        let StructBuilder { pairs, opts, .. } = self;
        let mut avx = true;
        let mut print = false;
        let mut simd = true;

        for MetaOpt { path, lit, .. } in opts {
            if path.is_ident("avx") {
                avx = lit.value
            } else if path.is_ident("print") {
                print = lit.value;
            } else if path.is_ident("simd") {
                simd = lit.value;
            } else {
                return Err(syn::Error::new(
                    path.span(),
                    format!("invalid attribute '{:?}'", path.get_ident()),
                ));
            }
        }

        Ok((avx, pairs.value(), print, simd))
    }
}
