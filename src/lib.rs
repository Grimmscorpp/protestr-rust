use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn provide(attr: TokenStream, item: TokenStream) -> TokenStream {
    panic!();
}
