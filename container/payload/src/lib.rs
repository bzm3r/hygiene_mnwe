use std::iter::once;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::PathSep,
    Path, PathArguments, PathSegment,
};

struct PathAttr(Option<Path>);

impl Parse for PathAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(PathAttr(if input.is_empty() {
            None
        } else {
            Some(input.parse::<Path>()?)
        }))
    }
}

#[proc_macro]
pub fn target(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = parse_macro_input!(input as PathAttr).0.unwrap_or(Path {
        leading_colon: Some(PathSep::default()),
        segments: Punctuated::from_iter(once(PathSegment {
            ident: Ident::new("container", Span::call_site()),
            arguments: PathArguments::None,
        })),
    });

    quote!(#path::hello_world::print()).into()
}
