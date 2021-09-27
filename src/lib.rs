use proc_macro::TokenStream;

/**
    Enclosed code is only included in dev builds (not when `--release` is specified)
*/
#[proc_macro]
pub fn if_debug(input: TokenStream) -> TokenStream {
    if cfg!(debug_assertions) {
        input
    } else {
        TokenStream::new()
    }
}
