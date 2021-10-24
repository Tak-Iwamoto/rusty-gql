mod error;
mod types;
mod utils;

use darling::ToTokens;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, parse_quote, AttributeArgs, Block, ItemFn};

use crate::utils::find_attr;

#[proc_macro_attribute]
pub fn GqlField(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.clone();
    let args = parse_macro_input!(args as AttributeArgs);
    let parent_type = find_attr(&args, "parent_type");
    let field = find_attr(&args, "field");

    // let attrs = parser.parse(args).unwrap();
    // let gql_attr = find_attr(&attrs[..], "parent_type");
    // dbg!(gql_attr);

    let mut item_fn = parse_macro_input!(input as ItemFn);
    let block = &item_fn.block;
    let ident = &item_fn.sig.ident;

    let new_stmt: Block = parse_quote! {{
        let value = #block;
        println!("{:?}", #parent_type);
        println!("{:?}", #field);
        format!("modified: {:?}", value)
    }};

    // let block = item_fn.block.as_ref();
    // let block: Block = parse_quote! {{
    //     println!("ident: {}", stringify!(#ident));
    //     let block = || #block;
    //     let ret = block();
    //     ret
    // }};
    *item_fn.block = new_stmt;
    item_fn.into_token_stream().into()
}
