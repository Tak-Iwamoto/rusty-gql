use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn parse_gql_object_input(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let st = match &input.data {
        syn::Data::Struct(s) => s,
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "GqlObject can be only applied to struct",
            ))
        }
    };

    let mut getters = Vec::new();
    // let mut resolvers = Vec::new();

    for field in &st.fields {
        let vis = &field.vis;
        let return_type = &field.ty;
        let field_ident = match &field.ident {
            Some(ident) => ident,
            None => return Err(syn::Error::new_spanned(field, "Must have type")),
        };

        getters.push(quote! {
            // convert to async method
            #vis async fn #field_ident(&self) -> rusty_gql::Response<#return_type> {
                ::std::result::Result::Ok(::std::clone::Clone::clone(&self.#field_ident))
            }
        });

        // resolvers.push(quote! {
        //     if ctx.current_field == #field_ident {
        //         let resolve_fn = async move {
        //             self.#field_ident.await
        //         };
        //         let obj = resolve_fn.await;
        //         return rusty_gql::Resolver::resolve(&obj, &ctx).await.map(::std::option::Option::Some);
        //     }
        // })
    }

    let expanded = quote! {
        #[allow(clippy::all, clippy::pedantic)]
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#getters)*
        }

        // #[rusty_gql::async_trait::async_trait]
        // impl #impl_generics rusty_gql::FieldResolver for #struct_name #ty_generics #where_clause {
        //     async fn resolve_field(&self, ctx: &rusty_gql::ExecutionContext) -> rusty_gql::Response<::std::option::Option<rusty_gql::GqlValue>> {
        //         #(#resolvers)*
        //         ::std::result::Result::Ok(::std::option::Option::None)
        //     }
        // }
    };
    Ok(expanded.into())
}
