mod error;
mod types;
mod utils;

use darling::ToTokens;
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::{AttributeArgs, Block, Ident, ImplItem, ItemImpl, ReturnType, parse_macro_input};

use crate::utils::{get_type_name, is_result_type};

#[proc_macro_attribute]
pub fn gql_object(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let mut item_impl = parse_macro_input!(input as ItemImpl);

    let expanded = parse_item_impl(&mut item_impl).unwrap();
    expanded
}

fn parse_item_impl(item_impl: &mut ItemImpl) -> Result<TokenStream, syn::Error> {
    let self_name = &item_impl.self_ty;

    let struct_name = get_type_name(&self_name)?;
    let shadow_type = Ident::new(&format!("__shadow{}", struct_name), Span::call_site());

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

            let block = &method.block;

            let is_result = is_result_type(return_type);

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

            let field_name = &method.sig.ident;

            let resolve_obj = quote! {{
                let res = self.#field_name(ctx).await;
            }};

            resolvers.push(quote! {
                {
                    if ctx.current_field.name == #field_name {
                        let resolve_fn = async move {
                            #resolve_obj
                        };

                        let obj = resolve_fn.await.map_err(|err| err)?;
                        return rusty_gql::Resolver::resolve(&obj, &ctx).await;
                    }
                }
            })
        }
    }
    let expanded = quote! {
        #item_impl

        #[allow(non_snake_case)]
        type #shadow_type<#generics_params> = #self_name;

        #[rusty_gql::async_trait::async_trait]
        impl #generics rusty_gql::FieldResolver for #shadow_type<#generics_params> #where_clause {
            async fn resolve_field(&self, ctx: &rusty_gql::ExecutionContext) -> rusty_gql::Response<::std::option::Option<rusty_gql::GqlValue>> {
                #(#resolvers)*
                ::std::result::Result::Ok(::std::option::Option::None)
            }
        }
    };
    Ok(expanded.into_token_stream().into())
}
