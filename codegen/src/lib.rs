mod object;
mod query;
mod types;
mod utils;

use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemImpl};

use crate::{object::parse_gql_object_input, query::parse_query_item_impl};

#[proc_macro_attribute]
pub fn GqlQuery(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let mut item_impl = parse_macro_input!(input as ItemImpl);

    let expanded = match parse_query_item_impl(&mut item_impl) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}

#[proc_macro_derive(GqlObject)]
pub fn derive_gql_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = match parse_gql_object_input(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}
