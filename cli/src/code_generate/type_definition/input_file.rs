use std::collections::HashSet;

use codegen::Scope;
use proc_macro2::TokenStream;
use rusty_gql::InputObjectType;
use syn::{__private::quote::quote, ext::IdentExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty};

pub struct InputObjectFile<'a> {
    pub filename: &'a str,
    pub def: &'a InputObjectType,
    pub path: &'a str,
}

impl<'a> InputObjectFile<'a> {
    pub async fn create_file(&self) -> Result<(), std::io::Error> {
        let path = self.path;
        match tokio::fs::File::open(&path).await {
            Ok(mut file) => {
                if self.filename == "mod.rs" {
                    let mut new_file = tokio::fs::File::create(&path).await?;
                    new_file
                        .write(new_file_content(self.def).as_bytes())
                        .await?;
                    Ok(())
                } else {
                    let mut current_file_src = String::new();
                    file.read_to_string(&mut current_file_src).await?;
                    let content = sync_input_file(&current_file_src, self.def);
                    let mut new_file = tokio::fs::File::create(&path).await?;
                    new_file.write(content.as_bytes()).await?;
                    Ok(())
                }
            }
            Err(_) => {
                let mut file = tokio::fs::File::create(&path).await?;
                file.write(new_file_content(self.def).as_bytes()).await?;
                Ok(())
            }
        }
    }
}

fn new_file_content(input_object_def: &InputObjectType) -> String {
    let mut scope = Scope::new();
    let struct_scope = scope.new_struct(&input_object_def.name).vis("pub");
    struct_scope.derive("GqlInputObject");

    for field in &input_object_def.fields {
        struct_scope.field(
            format!("pub {}", &field.name).as_str(),
            gql_value_ty_to_rust_ty(&field.meta_type),
        );
    }

    format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
}

fn sync_input_file(file_src: &str, input_object_def: &InputObjectType) -> String {
    let input_file_syntax = syn::parse_file(file_src).expect("Failed to parse a input file");
    let mut fields = Vec::new();
    let mut other_items = Vec::new();
    let mut struct_name: TokenStream = Default::default();
    let mut attributes: TokenStream = Default::default();
    for item in &input_file_syntax.items {
        if let syn::Item::Struct(struct_item) = item {
            let ident = &struct_item.ident;
            let struct_ident = ident.unraw().to_string();
            let attrs = &struct_item.attrs;
            struct_name = quote! {#ident};
            attributes = quote! {#(#attrs)*};
            if struct_ident.eq(&input_object_def.name) {
                let mut visited = HashSet::new();
                for field in &struct_item.fields {
                    let current_field_ident = field.ident.clone().unwrap().to_string();
                    if input_object_def
                        .fields
                        .iter()
                        .any(|f| f.name.eq(&current_field_ident))
                    {
                        fields.push(quote! {#field});
                    }
                    visited.insert(current_field_ident);
                }

                for schema_field in &input_object_def.fields {
                    if visited.contains(&schema_field.name) {
                        continue;
                    }
                    let field_str: TokenStream = format!(
                        "pub {}: {}",
                        &schema_field.name,
                        gql_value_ty_to_rust_ty(&schema_field.meta_type)
                    )
                    .parse()
                    .unwrap();
                    fields.push(field_str);
                }
            }
            continue;
        }

        if let syn::Item::Use(use_item) = item {
            if let syn::UseTree::Path(use_path) = &use_item.tree {
                let ident = use_path.ident.unraw().to_string();
                if ident.eq("rusty_gql") {
                    continue;
                }

                if ident.eq("crate") {
                    if let syn::UseTree::Path(child_path) = &*use_path.tree {
                        let ident = child_path.ident.unraw().to_string();
                        if ident.eq("graphql") {
                            continue;
                        }
                    }
                }
            }
            other_items.push(quote! {#item});
            continue;
        }

        other_items.push(quote! {#item});
    }

    let expanded = quote! {
        #![allow(warnings, unused)]
        use crate::graphql::*;
        use rusty_gql::*;

        #attributes
        pub struct #struct_name {
            #(#fields),*
        }
        #(#other_items)*
    };
    expanded.to_string()
}
