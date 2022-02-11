use std::collections::HashSet;

use codegen::Scope;
use proc_macro2::TokenStream;
use quote::quote;
use rusty_gql::EnumType;
use syn::ext::IdentExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::code_generate::{use_gql_definitions, util::is_default_item_use};
pub struct EnumFile<'a> {
    pub filename: &'a str,
    pub def: &'a EnumType,
    pub path: &'a str,
}

impl<'a> EnumFile<'a> {
    pub async fn create_file(&self) -> Result<(), std::io::Error> {
        let path = self.path;
        match tokio::fs::File::open(&path).await {
            Ok(mut file) => {
                let mut current_file_src = String::new();
                file.read_to_string(&mut current_file_src).await?;
                let content = sync_file(&current_file_src, self.def);
                let mut new_file = tokio::fs::File::create(&path).await?;
                new_file.write(content.as_bytes()).await?;
                Ok(())
            }
            Err(_) => {
                let mut file = tokio::fs::File::create(&path).await?;
                file.write(new_file_content(self.def).as_bytes()).await?;
                Ok(())
            }
        }
    }
}

fn new_file_content(enum_def: &EnumType) -> String {
    let mut scope = Scope::new();
    let enum_scope = scope.new_enum(&enum_def.name).vis("pub");
    enum_scope.derive("GqlEnum");
    enum_scope.derive("Copy");
    enum_scope.derive("Clone");
    enum_scope.derive("Eq");
    enum_scope.derive("PartialEq");

    for value in &enum_def.values {
        enum_scope.new_variant(&value.name);
    }

    format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
}

fn sync_file(file_src: &str, enum_def: &EnumType) -> String {
    let syntax = syn::parse_file(file_src).expect("Failed to parse a enum file");

    let mut variants = Vec::new();
    let mut use_items = Vec::new();
    let mut other_items = Vec::new();
    let enum_name: TokenStream = enum_def.name.parse().unwrap();
    let mut attributes: TokenStream = Default::default();

    for item in &syntax.items {
        if let syn::Item::Enum(enum_item) = item {
            let attrs = &enum_item.attrs;
            attributes = quote! {#(#attrs)*};
            let ident = &enum_item.ident;
            let enum_ident = ident.unraw().to_string();

            if enum_ident.eq(&enum_def.name) {
                let mut visited = HashSet::new();

                for variant in &enum_item.variants {
                    let variant_ident = variant.ident.clone().unraw().to_string();
                    if enum_def.values.iter().any(|v| v.name.eq(&variant_ident)) {
                        variants.push(quote! {#variant});
                    }
                    visited.insert(variant_ident);
                }

                for schema_variant in &enum_def.values {
                    if visited.contains(&schema_variant.name) {
                        continue;
                    }
                    variants.push(schema_variant.name.parse().unwrap());
                }
            }
            continue;
        }
        if let syn::Item::Use(item_use) = item {
            if !is_default_item_use(item_use) {
                use_items.push(quote! {#item});
            }
            continue;
        }

        other_items.push(quote! {#item});
    }

    let expanded = quote! {
        #![allow(warnings, unused)]
        use crate::graphql::*;
        use rusty_gql::*;
        #(#use_items)*

        #attributes
        pub enum #enum_name {
            #(#variants),*
        }
        #(#other_items)*
    };

    expanded.to_string()
}
