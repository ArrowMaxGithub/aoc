use proc_macro::TokenStream;

mod lut;

#[proc_macro]
pub fn lut(input: TokenStream) -> TokenStream {
    lut::expand(input)
}
