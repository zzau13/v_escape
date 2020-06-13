extern crate proc_macro;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::visit::Visit;

mod generator;
mod parser;

#[proc_macro_derive(Escape, attributes(escape))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    build(ast).parse().unwrap()
}

/// Takes a `syn::DeriveInput` and generates source code for it
///
/// Reads the metadata from the `escape()` attribute.
fn build(ast: &syn::DeriveInput) -> String {
    let (avx, pairs, print, simd) = visit_derive(ast);
    let code = generator::generate(&parser::parse(&pairs), simd, avx);

    if print {
        eprintln!("{}", code);
    }

    code
}

fn visit_derive(i: &syn::DeriveInput) -> Struct {
    StructBuilder::default().build(i)
}

type Struct = (bool, String, bool, bool);

struct StructBuilder {
    avx: bool,
    pairs: Option<String>,
    print: bool,
    simd: bool,
}

impl Default for StructBuilder {
    fn default() -> Self {
        StructBuilder {
            avx: true,
            pairs: None,
            print: false,
            simd: true,
        }
    }
}

impl StructBuilder {
    fn build(mut self, syn::DeriveInput { attrs, .. }: &syn::DeriveInput) -> Struct {
        for i in attrs {
            self.visit_meta(&i.parse_meta().expect("valid meta attributes"));
        }

        (
            self.avx,
            self.pairs.expect("Need pairs attribute"),
            self.print,
            self.simd,
        )
    }
}

impl<'a> Visit<'a> for StructBuilder {
    fn visit_meta_list(&mut self, syn::MetaList { path, nested, .. }: &'a syn::MetaList) {
        if path.is_ident("escape") {
            use syn::punctuated::Punctuated;
            for el in Punctuated::pairs(nested) {
                let it = el.value();
                self.visit_nested_meta(it)
            }
        }
    }

    fn visit_meta_name_value(
        &mut self,
        syn::MetaNameValue { path, lit, .. }: &'a syn::MetaNameValue,
    ) {
        if path.is_ident("avx") {
            if let syn::Lit::Bool(s) = lit {
                self.avx = s.value
            } else {
                panic!("attribute avx value must be boolean")
            }
        } else if path.is_ident("pairs") {
            if let syn::Lit::Str(s) = lit {
                self.pairs = Some(s.value());
            } else {
                panic!("attribute pairs must be string literal");
            }
        } else if path.is_ident("print") {
            if let syn::Lit::Bool(s) = lit {
                self.print = s.value;
            } else {
                panic!("attribute print must be boolean");
            }
        } else if path.is_ident("simd") {
            if let syn::Lit::Bool(s) = lit {
                self.simd = s.value;
            } else {
                panic!("attribute simd must be boolean");
            }
        } else {
            panic!("invalid attribute '{:?}'", path.get_ident())
        }
    }
}
