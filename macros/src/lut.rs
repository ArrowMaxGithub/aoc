use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::*;

pub fn expand(input: TokenStream) -> TokenStream {
    let intermediate = parse_macro_input!(input as Intermediate);
    quote! { #intermediate }.into()
}

struct Intermediate {}

impl Parse for Intermediate {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}

impl ToTokens for Intermediate {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Intermediate {} = self;

        tokens.extend(quote! {})
    }
}
