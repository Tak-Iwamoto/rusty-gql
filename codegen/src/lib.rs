mod error;
mod object;
mod resolver;
mod types;
mod utils;

use darling::{FromDeriveInput, ToTokens};
use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemImpl};

use crate::{
    object::{parse_gql_object_input, Object},
    resolver::parse_query_item_impl,
};

#[proc_macro_attribute]
pub fn Resolver(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let mut item_impl = parse_macro_input!(input as ItemImpl);

    let expanded = match parse_query_item_impl(&mut item_impl) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}

#[proc_macro_derive(Object)]
pub fn derive_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let args = match Object::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let expanded = match parse_gql_object_input(&args) {
        Ok(generated) => generated,
        Err(err) => err.to_token_stream().into(),
    };
    expanded
}
