mod error;
mod types;
mod utils;

use darling::ToTokens;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, parse_quote, AttributeArgs, Block, ImplItem, ItemFn, ItemImpl};

#[proc_macro_attribute]
pub fn gql_object(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let mut item_impl = parse_macro_input!(input as ItemImpl);

    // let mut item_fn = parse_macro_input!(input as ItemFn);
    // let block = &item_fn.block;
    // let ident = &item_fn.sig.ident;

    // let new_stmt: Block = parse_quote! {{
    //     let value = #block;
    //     println!("{:?}", #parent_type);
    //     println!("{:?}", #field);
    //     format!("modified: {:?}", value)
    // }};
    // *item_fn.block = new_stmt;

    let expanded = parse_item_impl(&mut item_impl).unwrap();

    expanded
}

fn parse_item_impl(item_impl: &mut ItemImpl) -> Result<TokenStream, syn::Error> {
    for item in &mut item_impl.items {
        if let ImplItem::Method(method) = item {
            if method.sig.asyncness.is_none() {
                return Err(
                    syn::Error::new_spanned(&method, "Resolver must be an async method.").into(),
                );
            }

            let field_ident = &method.sig.ident;

            let return_type = match &method.sig.output {
                syn::ReturnType::Default => {
                    return Err(syn::Error::new_spanned(
                        &method.sig.output,
                        "Resolver must have a return type",
                    )
                    .into());
                }
                syn::ReturnType::Type(_, ty) => ty,
            };

            let block = &method.block;

            let new_block = quote! {
                {
                    // ::std::result::Result::Ok(async move {
                    //     let value = #block;
                    //     value
                    // }.await)
                    let value = #block;
                    println!("{}", &value);
                    value
                }
            };
            method.block = syn::parse2::<Block>(new_block).expect("ItemImpl method is invalid.");
        }
    }
    Ok(item_impl.into_token_stream().into())
}
