use proc_macro::TokenStream;
use quote::quote;
use router::{Route, Router};

#[proc_macro_attribute]
pub fn add_api(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    _input
}
