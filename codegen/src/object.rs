use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, Block, FnArg, ImplItem, ItemImpl, ReturnType, Type, TypeReference};

use crate::utils::{get_method_args, is_result_type};

pub fn parse_object_item_impl(item_impl: &mut ItemImpl) -> Result<TokenStream, syn::Error> {
    let self_name = &item_impl.self_ty;

    let generics = &item_impl.generics;
    let generics_params = &generics.params;
    let where_clause = &generics.where_clause;
    let mut resolvers = Vec::new();
    for item in &mut item_impl.items {
        if let ImplItem::Method(method) = item {
            if method.sig.asyncness.is_none() {
                return Err(
                    syn::Error::new_spanned(&method, "Resolver must be an async method.").into(),
                );
            }

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

            let is_result = is_result_type(return_type);
            let block = &method.block;

            if !is_result {
                let result_block = quote! {
                    {
                        ::std::result::Result::Ok(async move {
                            let value = #block;
                            value
                        }.await)
                    }
                };
                method.block =
                    syn::parse2::<Block>(result_block).expect("ItemImpl method is invalid.");
                method.sig.output = syn::parse2::<ReturnType>(
                    quote! { -> ::std::result::Result<#return_type, String>},
                )
                .expect("ItemImpl return type is invalid.");
            }
            // let should_create_context = &method
            //     .sig
            //     .inputs
            //     .iter()
            //     .nth(1)
            //     .map(|x| {
            //         if let FnArg::Typed(pat) = x {
            //             if let Type::Reference(TypeReference { elem, .. }) = &*pat.ty {
            //                 if let Type::Path(path) = elem.as_ref() {
            //                     return path.path.segments.last().unwrap().ident != "FieldContext";
            //                 }
            //             }
            //         };
            //         true
            //     })
            //     .unwrap_or(true);

            // if *should_create_context {
            //     let arg_ctx = syn::parse2::<FnArg>(quote! { ctx: &rusty_gql::FieldContext<'_> })
            //         .expect("invalid arg type");
            //     method.sig.inputs.insert(1, arg_ctx);
            // }
            let method_name = &method.sig.ident;
            let field_name = method_name.unraw().to_string();

            let arg_idents = get_method_args(&method)?;
            let mut args = Vec::new();

            for arg in &arg_idents {
                args.push(quote! { #arg })
            }

            let resolve_obj = quote! {
                // let res = self.#method_name(ctx, #(#args),*).await;
                self.#method_name(ctx).await
            };

            resolvers.push(quote! {
                if ctx.item.name == #field_name {
                    let resolve_fn = async move {
                        #resolve_obj
                    };

                    let obj = resolve_fn.await.unwrap();
                    let selection_set = ctx.with_selection_set(&ctx.item.selection_set);

                    rusty_gql::resolve_selection_set(&obj, &selection_set, true).await.map(::std::option::Option::Some);
                }
            });
        }
    }

    let expanded = quote! {
        #item_impl

        #[rusty_gql::async_trait::async_trait]
        impl #generics rusty_gql::Resolver for #self_name #generics_params #where_clause {
            async fn resolve_field(&self, ctx: &rusty_gql::FieldContext<'_>) -> rusty_gql::Response<::std::option::Option<rusty_gql::GqlValue>> {
                #(#resolvers)*
                Ok(::std::option::Option::None)
            }
        }
    };
    // println!("{}", expanded.to_string());

    Ok(expanded.into())
}
