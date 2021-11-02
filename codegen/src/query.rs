// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{Block, Error, ItemFn, parse_quote, Type};

// pub fn generate(field_name: String, item_fn: &mut ItemFn) -> Result<TokenStream, syn::Error> {
//     let block = &item_fn.block;

//     let new_stmt: Block = parse_quote! {{
//         let value = #block;
//         println!("{:?}", #field_name);
//         format!("modified: {:?}", value)
//     }};

//     if item_fn.sig.asyncness.is_none() {
//         return Err(Error::new_spanned(&item_fn, "Must be asynchronous").into());
//     }

//     let return_type = match &item_fn.sig.output {
//         syn::ReturnType::Default => {
//             return Err(Error::new_spanned(&item_fn, "Must have return type").into());
//         },
//         syn::ReturnType::Type(token, ty) => {
//             parse_type(&ty);
//         },
//     }

//     // let block = item_fn.block.as_ref();
//     // let block: Block = parse_quote! {{
//     //     println!("ident: {}", stringify!(#ident));
//     //     let block = || #block;
//     //     let ret = block();
//     //     ret
//     // }};
//     *item_fn.block = new_stmt;
//     let expanded = quote! {
//         #item_fn
//     };
//     Ok(expanded.into())
// }

// // fn parse_type(ty: &Type) {
// //     if let Type::Path(ty_path) = ty {
// //         for seg in &ty_path.path.segments {
// //             seg.
// //         }

// //     }
// // }
