use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, Attribute, Generics, Ident, Type, Visibility};

use crate::error::CodegenResult;

#[derive(FromField)]
#[darling(attributes(gql), forward_attrs(doc))]
pub struct ModelField {
    pub ident: Option<Ident>,
    pub ty: Type,
    pub vis: Visibility,
    pub attrs: Vec<Attribute>,

    #[darling(default)]
    pub resolver: bool,
}

#[derive(FromDeriveInput)]
#[darling(attributes(gql), forward_attrs(doc))]
pub struct Model {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<Attribute>,
    pub data: Data<Ignored, ModelField>,
}

pub fn parse_gql_model_input(input: &Model) -> CodegenResult<TokenStream> {
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

        if field.resolver {
            resolvers.push(quote! {
            // 別途resolverが定義している時にここでさらに下のresolverに渡したい。
                if ctx.current_field.name == #field_name {
                    let resolve_fn = async move {
                        self.#field_ident().await
                    };
                    let obj = resolve_fn.await;
                    return rusty_gql::resolve_obj(&obj, &ctx).await.map(::std::option::Option::Some);
                }
            })
        } else {
            getters.push(quote! {
                // convert to async method
                #vis async fn #field_ident(&self) -> rusty_gql::Response<#return_type> {
                    ::std::result::Result::Ok(::std::clone::Clone::clone(&self.#field_ident))
                }
            });
        }
    }

    let expanded = quote! {
        #[allow(clippy::all, clippy::pedantic)]
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#getters)*
        }

        #[rusty_gql::async_trait::async_trait]
        impl #impl_generics rusty_gql::Resolver for #struct_name #ty_generics #where_clause {
            async fn resolve(&self, ctx: &rusty_gql::ExecutionContext<'_>) -> rusty_gql::Response<::std::option::Option<rusty_gql::GqlValue>> {
                #(#resolvers)*
                ::std::result::Result::Ok(::std::option::Option::None)
            }
        }
    };
    println!("{}", expanded.to_string());
    Ok(expanded.into())
}
