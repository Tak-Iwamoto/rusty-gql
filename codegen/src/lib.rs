mod error;
mod resolver;
mod types;
mod utils;

use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, ItemImpl};

use crate::resolver::parse_resolver_item_impl;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn GqlResolver(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(input as ItemImpl);

    let expanded = match parse_resolver_item_impl(&mut item_impl) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}
