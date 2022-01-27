use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, DeriveInput};

pub fn generate_scalar(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let self_ty = &derive_input.ident;
    let crate_name = quote! { rusty_gql };

    let type_name = self_ty.unraw().to_string();

    let (impl_generics, _, where_clause) = &derive_input.generics.split_for_impl();

    let expanded = quote! {
        #[#crate_name::async_trait::async_trait]
        impl #impl_generics #crate_name::FieldResolver for #self_ty #where_clause {
            async fn resolve_field(&self, ctx: &#crate_name::Context<'_>) -> #crate_name::ResolverResult<::std::option::Option<#crate_name::GqlValue>> {
                Ok(Some(self.into_gql_value()))
            }
            fn type_name() -> String {
                #type_name.to_string()
            }
        }

        impl #impl_generics #crate_name::CollectFields for #self_ty #where_clause {}

        #[#crate_name::async_trait::async_trait]
        impl #impl_generics #crate_name::SelectionSetResolver for #self_ty #where_clause {
            async fn resolve_selection_set(&self, ctx: &#crate_name::SelectionSetContext<'_>) -> #crate_name::ResolverResult<#crate_name::GqlValue> {
                Ok(self.into_gql_value())
            }
        }
    };

    Ok(expanded.into())
}
