mod error;
mod gql_struct;
mod resolver;
mod types;
mod utils;

use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemImpl};

use crate::{
    gql_struct::{parse_gql_struct_input, GqlStruct},
    resolver::parse_resolver_item_impl,
};

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

#[proc_macro_derive(GqlStruct)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let args = match GqlStruct::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let expanded = match parse_gql_struct_input(&args) {
        Ok(generated) => generated,
        Err(err) => err.to_token_stream().into(),
    };
    expanded
}
