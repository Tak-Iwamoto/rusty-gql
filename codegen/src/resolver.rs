use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, Block, FnArg, ImplItem, ItemImpl, ReturnType};

use crate::utils::{get_method_args_without_context, is_context_type, is_result_type};

pub fn generate_gql_resolver(item_impl: &mut ItemImpl) -> Result<TokenStream, syn::Error> {
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
                    quote! { -> ::std::result::Result<#return_type, rusty_gql::GqlError>},
                )
                .expect("ItemImpl return type is invalid.");
            }

            let is_contain_context = &method
                .sig
                .inputs
                .iter()
                .find(|arg| is_context_type(arg))
                .is_some();

            if !*is_contain_context {
                let arg_ctx = syn::parse2::<FnArg>(quote! { ctx: &rusty_gql::FieldContext<'_> })
                    .expect("invalid arg type");
                method.sig.inputs.insert(1, arg_ctx);
            }
            let method_name = &method.sig.ident;
            let field_name = method_name.unraw().to_string();

            let method_args = get_method_args_without_context(&method)?;
            let mut args = Vec::new();
            let mut gql_arg_values = Vec::new();

            for (arg_ident, ty) in method_args {
                args.push(quote! { #arg_ident });
                let ident = arg_ident.ident;
                let name = ident.unraw().to_string();
                gql_arg_values.push(quote! {
                    let #ident = ctx.get_arg_value::<#ty>(#name)?;
                });
            }

            resolvers.push(quote! {
                if ctx.item.name == #field_name {
                    let resolve_fn = async move {
                        #(#gql_arg_values)*
                        self.#method_name(ctx, #(#args),*).await
                    };

                    // TODO: error handling
                    let obj = resolve_fn.await.unwrap();
                    let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);
                    return obj.resolve_selection_set(&ctx_selection_set).await.map(Some);
                }
            });
        }
    }

    let expanded = quote! {
        #item_impl

        #[rusty_gql::async_trait::async_trait]
        impl #generics rusty_gql::Resolver for #self_name #generics_params #where_clause {
            async fn resolve_field(&self, ctx: &rusty_gql::FieldContext<'_>) -> rusty_gql::ResolverResult<::std::option::Option<rusty_gql::GqlValue>> {
                #(#resolvers)*
                Ok(::std::option::Option::None)
            }
        }

        #[rusty_gql::async_trait::async_trait]
        impl #generics rusty_gql::SelectionSetResolver for #self_name #generics_params #where_clause {
            async fn resolve_selection_set(&self, ctx: &rusty_gql::SelectionSetContext<'_>) -> rusty_gql::ResolverResult<rusty_gql::GqlValue> {
                ctx.resolve_selection_parallelly(self).await
            }
        }
    };

    Ok(expanded.into())
}
