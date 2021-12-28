mod resolver;
mod utils;

use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, ItemImpl};

use crate::resolver::generate_gql_resolver;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn GqlResolver(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(input as ItemImpl);

    let expanded = match generate_gql_resolver(&mut item_impl) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}
