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
    let mut into_gql_values = Vec::new();
    let mut from_gql_values = Vec::new();

    for variant in &enum_data.variants {
        let enum_value_ident = &variant.ident;
        let variant_str = enum_value_ident.unraw().to_string();

        resolve_fields.push(quote! {
            #self_ty::#enum_value_ident => Ok(Some(GqlValue::Enum(#variant_str.to_string())))
        });

        resolve_selection_sets.push(quote! {
            #self_ty::#enum_value_ident => Ok(GqlValue::Enum(#variant_str.to_string()))
        });

        into_gql_values.push(quote! {
            #self_ty::#enum_value_ident => GqlValue::Enum(#variant_str.to_string())
        });

        from_gql_values.push(quote! {
            if enum_value == #variant_str.to_string() {
                return Ok(#self_ty::#enum_value_ident)
            }
        })
    }

    let expanded = quote! {
        impl #impl_generics #crate_name::VariableType for #self_ty #where_clause {
            fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
                match value {
                    Some(v) => {
                        let enum_value = match v {
                            GqlValue::String(s) => s,
                            GqlValue::Enum(enu) => enu,
                            invalid_value => {
                                return Err(format!(
                                    "Expected type: enum, but found {}",
                                    invalid_value.to_string()
                                ));
                            }
                        };
                        #(#from_gql_values)*
                        Err(format!("{} is not contained", enum_value))
                    }
                    None => Err("Expected type: enum, but not found".to_string()),
                }
            }

            fn into_gql_value(&self) -> GqlValue {
                match self {
                    #(#into_gql_values),*
                }
            }
        }

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

        impl #impl_generics #crate_name::CollectFields for #self_ty #where_clause {}

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
