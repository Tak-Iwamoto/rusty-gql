use std::collections::{HashMap, HashSet};

use codegen::Scope;
use proc_macro2::TokenStream;
use quote::quote;
use rusty_gql::{FieldType, InterfaceType};
use syn::ext::IdentExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::code_generate::{
    use_gql_definitions,
    util::{gql_value_ty_to_rust_ty, is_default_item_use},
};

pub struct InterfaceFile<'a> {
    pub filename: &'a str,
    pub def: &'a InterfaceType,
    pub path: &'a str,
    pub interface_obj_map: &'a HashMap<String, Vec<String>>,
}

impl<'a> InterfaceFile<'a> {
    pub async fn create_file(&self) -> Result<(), std::io::Error> {
        let path = self.path;
        match tokio::fs::File::open(&path).await {
            Ok(mut file) => {
                let mut current_file_src = String::new();
                file.read_to_string(&mut current_file_src).await?;
                let content = sync_file(&current_file_src, self);
                let mut new_file = tokio::fs::File::create(&path).await?;
                new_file.write(content.as_bytes()).await?;
                Ok(())
            }
            Err(_) => {
                let mut file = tokio::fs::File::create(&path).await?;
                file.write(new_file_content(self).as_bytes()).await?;
                Ok(())
            }
        }
    }
}

fn new_file_content(interface_file: &InterfaceFile) -> String {
    let mut scope = Scope::new();
    let interface_name = &interface_file.def.name;
    let interface_scope = scope.new_enum(interface_name).vis("pub");
    interface_scope.derive("GqlInterface");
    interface_scope.derive("Clone");

    if let Some(impl_objects) = interface_file.interface_obj_map.get(interface_name) {
        for obj_name in impl_objects {
            interface_scope.new_variant(format!("{}({})", obj_name, obj_name).as_str());
        }
    }

    let mut impl_scope = Scope::new();
    let interface_impl = impl_scope.new_impl(interface_name);
    interface_impl.r#macro("#[GqlType(interface)]");

    for field in &interface_file.def.fields {
        let fn_scope = interface_impl.new_fn(&field.name);
        fn_scope.arg_ref_self();
        fn_scope.set_async(true);
        fn_scope.arg("ctx", "&Context<'_>");
        for arg in &field.arguments {
            fn_scope.arg(&arg.name, gql_value_ty_to_rust_ty(&arg.meta_type));
        }
        fn_scope.ret(format!(
            "Result<{}, Error>",
            gql_value_ty_to_rust_ty(&field.meta_type)
        ));
        fn_scope.line("match self {");
        if let Some(impl_objects) = interface_file.interface_obj_map.get(interface_name) {
            for obj_name in impl_objects {
                let args = &field
                    .arguments
                    .iter()
                    .map(|arg| arg.name.clone())
                    .collect::<Vec<_>>()
                    .join(",");
                fn_scope.line(format!(
                    "{interface_name}::{obj_name}(obj) => {{obj.{field_name}(&ctx, {args}).await}}",
                    interface_name = interface_name,
                    obj_name = &obj_name,
                    field_name = &field.name,
                    args = args
                ));
            }
        }
        fn_scope.line("}");
    }
    format!(
        "{}\n\n{}\n\n{}",
        use_gql_definitions(),
        scope.to_string(),
        impl_scope.to_string()
    )
}

fn sync_file(file_src: &str, interface_file: &InterfaceFile) -> String {
    let syntax = syn::parse_file(file_src).expect("Failed to parse a input file");
    let mut variants = Vec::new();
    let mut use_items = Vec::new();
    let mut other_items = Vec::new();
    let mut current_impl_items = Vec::new();
    let mut new_impl_items = Vec::new();
    let enum_name: TokenStream = interface_file.def.name.parse().unwrap();
    let mut enum_attrs: TokenStream = Default::default();
    let mut impl_attrs: TokenStream = Default::default();
    let interface_name = &interface_file.def.name;
    let implement_objects = interface_file
        .interface_obj_map
        .get(interface_name)
        .unwrap();

    for item in &syntax.items {
        if let syn::Item::Enum(enum_item) = item {
            let attrs = &enum_item.attrs;
            enum_attrs = quote! {#(#attrs)*};
            let ident = &enum_item.ident;
            let enum_ident = ident.unraw().to_string();

            if enum_ident.eq(interface_name) {
                let mut visited_variants = HashSet::new();
                for variant in &enum_item.variants {
                    let variant_ident = variant.ident.clone().unraw().to_string();
                    if implement_objects.contains(&variant_ident) {
                        variants.push(quote! {#variant});
                    }
                    visited_variants.insert(variant_ident);
                }

                for schema_variant in implement_objects {
                    if visited_variants.contains(schema_variant) {
                        continue;
                    }
                    let variant_str = format!("{variant}({variant})", variant = schema_variant);
                    variants.push(variant_str.parse().unwrap());
                }
            }
            continue;
        }

        if let syn::Item::Impl(item_impl) = item {
            let attrs = &item_impl.attrs;
            impl_attrs = quote! {#(#attrs)*};
            let mut visited_fields = HashSet::new();
            for item in &item_impl.items {
                if let syn::ImplItem::Method(item_method) = item {
                    let method_name = item_method.sig.ident.unraw().to_string();
                    let schema_field = get_field_by_name(interface_file.def, &method_name);
                    if let Some(_field) = schema_field {
                        current_impl_items.push(item);
                    }
                    visited_fields.insert(method_name);
                }
            }

            for schema_field in &interface_file.def.fields {
                if visited_fields.contains(&schema_field.name) {
                    continue;
                }
                let mut method_args = vec!["&self".to_string(), "ctx: &Context<'_>".to_string()];
                let mut variant_method_args: Vec<TokenStream> = vec!["&ctx".parse().unwrap()];
                for schema_arg in &schema_field.arguments {
                    let arg_str = format!(
                        "{}: {}",
                        schema_arg.name,
                        gql_value_ty_to_rust_ty(&schema_arg.meta_type)
                    );
                    method_args.push(arg_str);
                    variant_method_args.push(schema_arg.name.parse().unwrap());
                }

                let method_name: TokenStream = schema_field.name.parse().unwrap();

                let mut variant_methods = Vec::new();
                for schema_variant in implement_objects {
                    let expanded = quote! {
                        #interface_name::#schema_variant(obj) => obj.#method_name(#(#variant_method_args),*).await;
                    };
                    variant_methods.push(expanded);
                }

                let return_ty = gql_value_ty_to_rust_ty(&schema_field.meta_type);
                let method_args_quote: TokenStream = method_args.join(",").parse().unwrap();
                let expanded = quote! {
                    async fn #method_name(#method_args_quote) -> Result<#return_ty, Error>{
                        match self {
                            #(#variant_methods),*
                        }
                    }
                };
                new_impl_items.push(expanded);
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

        #enum_attrs
        pub enum #enum_name {
            #(#variants),*
        }

        #impl_attrs
        impl #enum_name {
            #(#current_impl_items)*
            #(#new_impl_items)*
        }

        #(#other_items)*
    };
    expanded.to_string()
}

fn get_field_by_name<'a>(interface: &'a InterfaceType, field_name: &str) -> Option<&'a FieldType> {
    interface.fields.iter().find(|f| f.name.eq(field_name))
}
