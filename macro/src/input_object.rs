use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, DeriveInput};

pub fn generate_input_object(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let self_ty = &derive_input.ident;
    let crate_name = quote! { rusty_gql };

    let type_name = self_ty.unraw().to_string();

    let (impl_generics, _, where_clause) = &derive_input.generics.split_for_impl();

    let struct_data = match &derive_input.data {
        syn::Data::Struct(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Input Object type must be struct type",
            ));
        }
    };

    let mut fields = Vec::new();
    let mut get_fields = Vec::new();
    let mut set_fields = Vec::new();
    for field in &struct_data.fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let field_name = ident.unraw().to_string();

        get_fields.push(quote! {
            let #ident: #ty = #crate_name::GqlInputType::from_gql_value(obj.get(#field_name).cloned())?;
        });
        fields.push(ident);

        set_fields.push(quote! {
            obj.insert(#field_name.to_string(), #crate_name::GqlInputType::into_gql_value(&self.#ident));
        })
    }

    let expanded = quote! {
        impl #impl_generics #crate_name::GqlInputType for #self_ty #where_clause {
            fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
                if let Some(GqlValue::Object(obj)) = value {
                    #(#get_fields)*
                    Ok(Self { #(#fields),* })
                } else {
                    Err("Invalid type, Expected type: object".to_string())
                }
            }

            fn into_gql_value(&self) -> GqlValue {
                let mut obj = std::collections::BTreeMap::new();
                #(#set_fields)*
                #crate_name::GqlValue::Object(obj)
            }
        }

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
