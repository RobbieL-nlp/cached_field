mod field;
mod utils;
use field::cached_field_impl;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn cached_field(args: TokenStream, item: TokenStream) -> TokenStream {
    cached_field_impl(args, item)
}
