use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, Block, FnArg, ImplItem, ItemImpl, NestedMeta, ReturnType};

use crate::utils::{
    get_method_args_without_context, is_context_type, is_interface, is_internal, is_result_type,
};

pub fn generate_type(
    item_impl: &mut ItemImpl,
    args: &[NestedMeta],
) -> Result<TokenStream, syn::Error> {
    let self_ty = &item_impl.self_ty;
    let crate_name = if is_internal(&args) {
        quote! { crate }
    } else {
        quote! { rusty_gql }
    };

    let type_name = match self_ty.as_ref() {
        syn::Type::Path(path) => path.path.segments.last().unwrap().ident.unraw().to_string(),
        _ => {
            return Err(syn::Error::new_spanned(&self_ty, "Invalid struct").into());
        }
    };

    let (impl_generics, _, where_clause) = &item_impl.generics.split_for_impl();

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
                    quote! { -> ::std::result::Result<#return_type, #crate_name::ErrorWrapper>},
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
                let arg_ctx = syn::parse2::<FnArg>(quote! { ctx: &#crate_name::FieldContext<'_> })
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
                        let res = self.#method_name(ctx, #(#args),*).await;
                        res.map_err(|err| #crate_name::ErrorWrapper::from(err).into_gql_error(ctx.item.position))
                    };

                    let obj = resolve_fn.await?;
                    let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);
                    return obj.resolve_selection_set(&ctx_selection_set).await.map(Some);
                }
            });
        }
    }

    let collect_fields = if is_interface(&args) {
        None
    } else {
        Some(quote! {
            impl #impl_generics #crate_name::CollectFields for #self_ty #where_clause {}
        })
    };

    let expanded = quote! {
        #item_impl

        #[#crate_name::async_trait::async_trait]
        impl #impl_generics #crate_name::FieldResolver for #self_ty #where_clause {
            async fn resolve_field(&self, ctx: &#crate_name::FieldContext<'_>) -> #crate_name::ResolverResult<::std::option::Option<#crate_name::GqlValue>> {
                #(#resolvers)*
                Ok(::std::option::Option::None)
            }
            fn type_name() -> String {
                #type_name.to_string()
            }
        }

        #collect_fields

        #[#crate_name::async_trait::async_trait]
        impl #impl_generics #crate_name::SelectionSetResolver for #self_ty #where_clause {
            async fn resolve_selection_set(&self, ctx: &#crate_name::SelectionSetContext<'_>) -> #crate_name::ResolverResult<#crate_name::GqlValue> {
                #crate_name::resolve_selection_parallelly(ctx, self).await
            }
        }
    };

    Ok(expanded.into())
}
