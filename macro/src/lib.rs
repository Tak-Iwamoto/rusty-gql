mod enum_type;
mod input_object;
mod interface;
mod scalar;
mod ty;
mod union;
mod utils;

use enum_type::generate_enum;
use input_object::generate_input_object;
use interface::generate_interface;
use proc_macro::{self, TokenStream};
use scalar::generate_scalar;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemImpl};
use union::generate_union;

use crate::ty::generate_type;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn GqlType(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(input as ItemImpl);
    let args = parse_macro_input!(args as AttributeArgs);

    let expanded = match generate_type(&mut item_impl, &args[..]) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}

#[proc_macro_derive(GqlScalar)]
pub fn scalar_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    let expanded = match generate_scalar(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}

#[proc_macro_derive(GqlUnion)]
pub fn union_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    let expanded = match generate_union(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}

#[proc_macro_derive(GqlInterface)]
pub fn interface_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    let expanded = match generate_interface(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}

#[proc_macro_derive(GqlEnum)]
pub fn enum_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    let expanded = match generate_enum(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}

#[proc_macro_derive(GqlInputObject)]
pub fn input_object_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    let expanded = match generate_input_object(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}
