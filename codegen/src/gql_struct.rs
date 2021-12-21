use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, Attribute, Generics, Ident, Type, Visibility};

use crate::error::CodegenResult;

#[derive(FromField)]
#[darling(attributes(gql), forward_attrs(doc))]
pub struct GqlStructField {
    pub ident: Option<Ident>,
    pub ty: Type,
    pub vis: Visibility,
    pub attrs: Vec<Attribute>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(gql), forward_attrs(doc))]
pub struct GqlStruct {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<Attribute>,
    pub data: Data<Ignored, GqlStructField>,
}

pub fn parse_gql_struct_input(input: &GqlStruct) -> CodegenResult<TokenStream> {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let st = match &input.data {
        Data::Struct(s) => s,
        // _ => return Err(CodegenError::Syn(syn::Error::new_spanned(&input.to_owned(), "test"))),
        _ => unreachable!(),
    };

    let mut getters = Vec::new();
    let mut resolvers = Vec::new();

    for field in &st.fields {
        let vis = &field.vis;
        let return_type = &field.ty;
        let field_ident = match &field.ident {
            Some(ident) => ident,
            // None => return Err(syn::Error::new_spanned(field, "Must have type")),
            None => unreachable!(),
        };

        let field_name = field_ident.unraw().to_string();

        resolvers.push(quote! {
            if ctx.item.name == #field_name {
                let resolve_fn = async move {
                    self.#field_ident().await
                };
                // TODO: error handling
                let obj = resolve_fn.await.unwrap();
                let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);
                return ctx_selection_set.resolve_selection_parallelly(&obj).await.map(Some);
            }
        });

        getters.push(quote! {
            // convert to async method
            #vis async fn #field_ident(&self) -> rusty_gql::ResolverResult<#return_type> {
                ::std::result::Result::Ok(::std::clone::Clone::clone(&self.#field_ident))
            }
        });
    }

    let expanded = quote! {
        #[allow(clippy::all, clippy::pedantic)]
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#getters)*
        }

        #[rusty_gql::async_trait::async_trait]
        impl #impl_generics rusty_gql::Resolver for #struct_name #ty_generics #where_clause {
            async fn resolve_field(&self, ctx: &rusty_gql::FieldContext<'_>) -> rusty_gql::ResolverResult<::std::option::Option<rusty_gql::GqlValue>> {
                #(#resolvers)*
                Ok(::std::option::Option::None)
            }
        }

        #[rusty_gql::async_trait::async_trait]
        impl #impl_generics rusty_gql::SelectionSetResolver for #struct_name #ty_generics #where_clause {
            async fn resolve_selection_set(&self, ctx: &rusty_gql::SelectionSetContext<'_>) -> rusty_gql::ResolverResult<rusty_gql::GqlValue> {
                ctx.resolve_selection_parallelly(self).await
            }
        }
    };
    Ok(expanded.into())
}
