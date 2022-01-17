use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, DeriveInput};

pub fn generate_union(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let self_ty = &derive_input.ident;
    let crate_name = quote! { rusty_gql };

    let type_name = self_ty.unraw().to_string();

    let (impl_generics, _, where_clause) = &derive_input.generics.split_for_impl();

    let union_data = match &derive_input.data {
        syn::Data::Enum(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Union type must be enum rust type",
            ));
        }
    };

    let mut introspection_type_names = Vec::new();
    let mut collect_all_fields = Vec::new();

    for variant in &union_data.variants {
        let enum_value_ident = &variant.ident;

        introspection_type_names.push(quote! {
            #self_ty::#enum_value_ident(obj) => obj.introspection_type_name()
        });

        collect_all_fields.push(quote! {
            #self_ty::#enum_value_ident(obj) => obj.collect_all_fields(ctx, fields)
        })
    }

    let expanded = quote! {
        #[#crate_name::async_trait::async_trait]
        impl #impl_generics #crate_name::FieldResolver for #self_ty #where_clause {
            async fn resolve_field(&self, ctx: &#crate_name::FieldContext<'_>) -> #crate_name::ResolverResult<::std::option::Option<#crate_name::GqlValue>> {
                Ok(None)
            }
            fn type_name() -> String {
                #type_name.to_string()
            }

            fn introspection_type_name(&self) -> String {
                match self {
                    #(#introspection_type_names),*
                }
            }

            fn collect_all_fields<'union, 'ctx: 'union>(
                &'union self,
                ctx: &SelectionSetContext<'ctx>,
                fields: &mut Fields<'union>,
            ) -> ResolverResult<()> {
                match self {
                    #(#collect_all_fields),*
                }
            }
        }

        #[#crate_name::async_trait::async_trait]
        impl #impl_generics #crate_name::SelectionSetResolver for #self_ty #where_clause {
            async fn resolve_selection_set(&self, ctx: &#crate_name::SelectionSetContext<'_>) -> #crate_name::ResolverResult<#crate_name::GqlValue> {
                #crate_name::resolve_selection_parallelly(ctx, self).await
            }
        }
    };

    Ok(expanded.into())
}
