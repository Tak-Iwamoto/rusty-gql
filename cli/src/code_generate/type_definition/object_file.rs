use std::collections::HashSet;

use codegen::{Scope, Type};
use proc_macro2::TokenStream;
use quote::quote;
use rusty_gql::{FieldType, ObjectType};
use syn::ext::IdentExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::code_generate::{
    use_gql_definitions,
    util::{gql_value_ty_to_rust_ty, is_default_item_use, is_gql_primitive_ty},
};

pub struct ObjectFile<'a> {
    pub filename: &'a str,
    pub def: &'a ObjectType,
    pub path: &'a str,
}

impl<'a> ObjectFile<'a> {
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

fn new_file_content(object_def: &ObjectType) -> String {
    let mut struct_scope_base = Scope::new();
    let struct_name = &object_def.name;
    let struct_scope = struct_scope_base
        .new_struct(&struct_name.to_string())
        .vis("pub");
    struct_scope.derive("Clone");

    let mut impl_scope = Scope::new();
    let struct_imp = impl_scope.new_impl(&struct_name.to_string());
    struct_imp.r#macro("#[GqlType]");

    for field in &object_def.fields {
        let field_name = &field.name;
        let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
        if is_return_primitive_ty(field) {
            struct_scope.field(format!("pub {}", &field_name).as_str(), &return_ty);
        }

        let fn_scope = struct_imp.new_fn(field_name);
        fn_scope.arg("ctx", "&Context<'_>");
        for arg in &field.arguments {
            fn_scope.arg(&arg.name, gql_value_ty_to_rust_ty(&arg.meta_type));
        }

        fn_scope.arg_ref_self();
        fn_scope.set_async(true);
        fn_scope.vis("pub");

        fn_scope.ret(Type::new(&return_ty));

        let block_str = build_block_str(field, field_name);
        fn_scope.line(block_str);
    }

    format!(
        "{}\n\n{}\n\n{}",
        use_gql_definitions(),
        struct_scope_base.to_string(),
        impl_scope.to_string()
    )
}

fn sync_file(file_src: &str, object_def: &ObjectType) -> String {
    let syntax = syn::parse_file(file_src).expect("Failed to parse a input file");
    let mut fields = Vec::new();
    let mut use_items = Vec::new();
    let mut other_items = Vec::new();
    let mut current_impl_items = Vec::new();
    let mut new_impl_items = Vec::new();
    let mut struct_name: TokenStream = Default::default();
    let mut struct_attributes: TokenStream = Default::default();
    let mut impl_attributes: TokenStream = Default::default();
    for item in &syntax.items {
        if let syn::Item::Struct(item_struct) = item {
            let ident = &item_struct.ident;
            let struct_ident = ident.unraw().to_string();
            let attrs = &item_struct.attrs;
            struct_name = quote! {#ident};
            struct_attributes = quote! {#(#attrs)*};
            if struct_ident.eq(&object_def.name) {
                let mut visited_fields = HashSet::new();
                for field in &item_struct.fields {
                    let current_field_ident = field.ident.clone().unwrap().to_string();
                    if get_field_by_name(object_def, &current_field_ident).is_some() {
                        fields.push(quote! {#field});
                    }
                    visited_fields.insert(current_field_ident);
                }

                for schema_field in &object_def.fields {
                    if visited_fields.contains(&schema_field.name) {
                        continue;
                    }
                    if is_return_primitive_ty(schema_field) {
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
            }
            continue;
        }

        if let syn::Item::Impl(item_impl) = item {
            let attrs = &item_impl.attrs;
            impl_attributes = quote! {#(#attrs)*};
            let mut visited_fields = HashSet::new();
            for item in &item_impl.items {
                if let syn::ImplItem::Method(item_method) = item {
                    let method_name = item_method.sig.ident.unraw().to_string();
                    let schema_field = get_field_by_name(object_def, &method_name);
                    if let Some(_field) = schema_field {
                        current_impl_items.push(item);
                    }
                    visited_fields.insert(method_name);
                }
            }

            for schema_field in &object_def.fields {
                if visited_fields.contains(&schema_field.name) {
                    continue;
                }
                if !is_return_primitive_ty(schema_field) {
                    let mut args = vec!["&self".to_string(), "ctx: &Context<'_>".to_string()];
                    for schema_arg in &schema_field.arguments {
                        let arg_str = format!(
                            "{}: {}",
                            schema_arg.name,
                            gql_value_ty_to_rust_ty(&schema_arg.meta_type)
                        );
                        args.push(arg_str);
                    }
                    let impl_fn: TokenStream = format!(
                        "pub async fn {}({}) -> {} {{ todo!() }}",
                        &schema_field.name,
                        args.join(","),
                        gql_value_ty_to_rust_ty(&schema_field.meta_type)
                    )
                    .parse()
                    .unwrap();
                    new_impl_items.push(impl_fn);
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

        #struct_attributes
        pub struct #struct_name {
            #(#fields),*
        }

        #impl_attributes
        impl #struct_name {
            #(#current_impl_items)*
            #(#new_impl_items)*
        }

        #(#other_items)*
    };
    expanded.to_string()
}

fn is_return_primitive_ty(field: &FieldType) -> bool {
    is_gql_primitive_ty(field.meta_type.name())
}

fn is_copy_gql_ty(field: &FieldType) -> bool {
    vec!["Int", "Float", "Boolean"].contains(&field.meta_type.name())
}

fn build_block_str(field: &FieldType, name: &str) -> String {
    let block_str = if is_return_primitive_ty(field) {
        if is_copy_gql_ty(field) {
            format!("self.{}", &name)
        } else {
            format!("self.{}.clone()", &name)
        }
    } else {
        "todo!()".to_string()
    };
    block_str
}

fn get_field_by_name<'a>(object: &'a ObjectType, field_name: &str) -> Option<&'a FieldType> {
    object.fields.iter().find(|f| f.name.eq(field_name))
}
