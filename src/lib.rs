use proc_macro::TokenStream;

#[proc_macro]
pub fn if_debug(input: TokenStream) -> TokenStream {
    if cfg!(debug_assertions) {
        input
    } else {
        TokenStream::new()
    }
}
