extern crate proc_macro;
#[macro_use]
extern crate nom;

use proc_macro::TokenStream;
use quote::ToTokens;

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
    // Check that an attribute called `escape()` exists and that it is
    // the proper type (list).
    let mut meta = None;
    for attr in &ast.attrs {
        match attr.interpret_meta() {
            Some(m) => {
                if m.name() == "escape" {
                    meta = Some(m)
                }
            }
            None => {
                let mut tokens = quote::__rt::TokenStream::new();
                attr.to_tokens(&mut tokens);
                panic!("unable to interpret attribute: {}", tokens)
            }
        }
    }

    let meta_list = match meta.expect("no attribute 'escape' found") {
        syn::Meta::List(inner) => inner,
        _ => panic!("attribute 'escape' has incorrect type"),
    };

    let mut avx = true;
    let mut pairs = None;
    let mut print = false;
    let mut simd = true;
    let mut sized = false;
    for nm_item in meta_list.nested {
        if let syn::NestedMeta::Meta(ref item) = nm_item {
            if let syn::Meta::NameValue(ref pair) = item {
                match pair.ident.to_string().as_ref() {
                    name @ "avx" => {
                        if let syn::Lit::Str(ref s) = pair.lit {
                            avx = resolve_true(&s.value(), name);
                        }
                    }
                    "pairs" => {
                        if let syn::Lit::Str(ref s) = pair.lit {
                            pairs = Some(s.value());
                        }
                    }
                    name @ "print" => {
                        if let syn::Lit::Str(ref s) = pair.lit {
                            print = resolve_true(&s.value(), name);
                        }
                    }
                    name @ "simd" => {
                        if let syn::Lit::Str(ref s) = pair.lit {
                            simd = resolve_true(&s.value(), name);
                        }
                    }
                    name @ "sized" => {
                        if let syn::Lit::Str(ref s) = pair.lit {
                            sized = resolve_true(&s.value(), name);
                        }
                    }
                    attr => panic!("unsupported annotation key '{}' found", attr),
                }
            }
        }
    }

    let pairs: &str = &pairs.expect("");

    let mut pairs = parser::parse(pairs);
    // need order for calculate ranges
    pairs.sort_by(|a, b| a.char.cmp(&b.char));

    let code = generator::generate(&pairs, sized, simd, avx);

    if print {
        eprintln!("{}", code);
    }

    code
}

fn resolve_true(s: &str, name: &str) -> bool {
    match s {
        "true" => true,
        "false" => false,
        attr => panic!(
            "{} attribute need \"true\" or \"false\" and is \"{}\"",
            name, attr
        ),
    }
}
