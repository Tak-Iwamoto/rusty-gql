use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, Block, ImplItem, ItemImpl, ReturnType};

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

            let method_name = &method.sig.ident;
            let field_name = method_name.unraw().to_string();

            let arg_idents = get_method_args(&method)?;
            let mut args = Vec::new();

            for arg in &arg_idents {
                args.push(quote! { #arg })
            }

            let resolve_obj = quote! {{
                self.#method_name(#(#args),*).await
            }};

            resolvers.push(quote! {
                {
                    if &ctx.current_field.name == #field_name {
                        let resolve_fn = async move {
                            #resolve_obj
                        };

                        let obj = resolve_fn.await.map_err(|err| err).unwrap();
                        return rusty_gql::resolve_object(&obj, &ctx, true).await.map(::std::option::Option::Some);
                    }
                }
            });
        }
    }

    let expanded = quote! {
        #item_impl

        #[allow(non_snake_case)]

        #[rusty_gql::async_trait::async_trait]
        impl #generics rusty_gql::FieldResolver for #self_name #generics_params #where_clause {
            async fn resolve_field(&self, ctx: &rusty_gql::ExecutionContext) -> rusty_gql::Response<::std::option::Option<rusty_gql::GqlValue>> {
                #(#resolvers)*
                Ok(::std::option::Option::None)
            }
        }
    };

    Ok(expanded.into())
}
