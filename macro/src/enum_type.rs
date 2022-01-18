use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, DeriveInput};

pub fn generate_enum(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let self_ty = &derive_input.ident;
    let crate_name = quote! { rusty_gql };

    let type_name = self_ty.unraw().to_string();

    let (impl_generics, _, where_clause) = &derive_input.generics.split_for_impl();

    let enum_data = match &derive_input.data {
        syn::Data::Enum(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Enum type must be enum rust type",
            ));
        }
    };

    let mut resolve_fields = Vec::new();
    let mut resolve_selection_sets = Vec::new();

    for variant in &enum_data.variants {
        let enum_value_ident = &variant.ident;
        let variant_str = enum_value_ident.unraw().to_string();

        resolve_fields.push(quote! {
            #self_ty::#enum_value_ident => Ok(Some(GqlValue::Enum(#variant_str.to_string())))
        });

        resolve_selection_sets.push(quote! {
            #self_ty::#enum_value_ident => Ok(GqlValue::Enum(#variant_str.to_string()))
        })
    }

    let expanded = quote! {
        #[#crate_name::async_trait::async_trait]
        impl #impl_generics #crate_name::FieldResolver for #self_ty #where_clause {
            async fn resolve_field(&self, ctx: &#crate_name::FieldContext<'_>) -> #crate_name::ResolverResult<::std::option::Option<#crate_name::GqlValue>> {
                match self {
                    #(#resolve_fields),*
                }
            }
            fn type_name() -> String {
                #type_name.to_string()
            }
        }

        #[#crate_name::async_trait::async_trait]
        impl #impl_generics #crate_name::SelectionSetResolver for #self_ty #where_clause {
            async fn resolve_selection_set(&self, ctx: &#crate_name::SelectionSetContext<'_>) -> #crate_name::ResolverResult<#crate_name::GqlValue> {
                match self {
                    #(#resolve_selection_sets),*
                }
            }
        }
    };

    Ok(expanded.into())
}
