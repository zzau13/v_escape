use proc_macro::TokenStream;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Token;

mod generator;

/// Generate static tables and call macros
#[proc_macro]
pub fn derive(input: TokenStream) -> TokenStream {
    let Args {
        avx,
        mut pairs,
        print,
        simd,
    } = match syn::parse::<Builder>(input).and_then(Builder::build) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };

    // need order for calculate ranges
    pairs.sort_by_key(|p| p.ch);

    // check repeated
    for i in 0..pairs.len() - 1 {
        let p1 = &pairs[i];
        let p2 = &pairs[i + 1];
        assert!(!(p1.ch == p2.ch), "{:?} and {:?} are repeated", p1, p2);
    }

    let code = generator::generate(&pairs, simd, avx);

    if print {
        eprintln!("{}", code);
    }

    code.parse().unwrap()
}

#[derive(Debug)]
pub(crate) struct Pair {
    ch: u8,
    quote: String,
}

impl Pair {
    pub fn new<I: Into<String>>(ch: u8, quote: I) -> Self {
        Pair {
            ch,
            quote: quote.into(),
        }
    }
}
/// Proc macro arguments data
struct Args {
    pairs: Vec<Pair>,
    avx: bool,
    print: bool,
    simd: bool,
}

/// Key-value argument
struct MetaOpt<Lit: Parse> {
    path: syn::Path,
    _eq_token: Token![=],
    lit: Lit,
}

impl<Lit: Parse> Parse for MetaOpt<Lit> {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            _eq_token: input.parse()?,
            lit: input.parse()?,
        })
    }
}

struct Ch(u8);

impl Ch {
    fn new(n: u8) -> Result<Self, ()> {
        match n < (std::i8::MAX as u8) {
            true => Ok(Ch(n)),
            false => Err(()),
        }
    }
}

impl Parse for Ch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::Lit;
        let lit: &Lit = &input.parse()?;
        let map_err = |_: ()| syn::Error::new(lit.span(), "Only accept ASCII characters");
        Ok(match lit {
            Lit::Byte(v) => Ch::new(v.value()).map_err(map_err)?,
            Lit::Char(v) => Ch::new(v.value() as u8).map_err(map_err)?,
            Lit::Int(v) => v
                .base10_parse::<i8>()
                .map_err(|_| ())
                .and_then(|x| Ch::new(x as u8))
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

/// Proc macro arguments parser
struct Builder {
    pairs: Punctuated<PairBuilder, Token![,]>,
    _sep: Option<Token![;]>,
    opts: Punctuated<MetaOpt<syn::LitBool>, Token![,]>,
}

impl Parse for Builder {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> syn::Result<Self> {
        Ok(Self {
            pairs: Punctuated::parse_separated_nonempty(input)?,
            _sep: input.parse()?,
            opts: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Builder {
    /// Consume and return arguments data
    fn build(self) -> syn::Result<Args> {
        let Builder { pairs, opts, .. } = self;
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

        Ok(Args {
            pairs: pairs
                .into_pairs()
                .map(|x| x.into_value())
                .map(|x| Pair::new(x.ch.0, x.quote.value()))
                .collect(),
            avx,
            print,
            simd,
        })
    }
}
