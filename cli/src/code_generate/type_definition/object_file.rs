use std::collections::BTreeMap;

use codegen::{Scope, Type};
use heck::ToSnakeCase;
use rusty_gql::{GqlField, GqlInterface, GqlObject};

use crate::code_generate::{
    use_gql_definitions,
    util::{gql_value_ty_to_rust_ty, is_gql_primitive_ty},
    FileDefinition,
};

pub struct ObjectFile<'a> {
    pub def: &'a GqlObject,
    pub path: &'a str,
    pub interfaces_map: &'a BTreeMap<String, GqlInterface>,
}

impl<'a> FileDefinition for ObjectFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut struct_scope_base = Scope::new();
        let struct_name = &self.def.name;
        let struct_scope = struct_scope_base
            .new_struct(&struct_name.to_string())
            .vis("pub");

        let mut implemented_fields = Vec::new();
        let mut impl_str = Vec::new();
        for interface in &self.def.implements_interfaces {
            let mut scope = Scope::new();
            let impl_interface = scope.new_impl(&struct_name);
            impl_interface.impl_trait(interface);
            impl_interface.r#macro("#[async_trait::async_trait]");

            if let Some(gql_interface) = self.interfaces_map.get(interface) {
                for field in &gql_interface.fields {
                    let field_name = &field.name.to_snake_case();
                    let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
                    let fn_scope = impl_interface.new_fn(&field_name);
                    fn_scope.set_async(true);
                    fn_scope.arg_ref_self();

                    implemented_fields.push(field_name.to_string());
                    for arg in &field.arguments {
                        fn_scope.arg(
                            &arg.name.to_snake_case(),
                            gql_value_ty_to_rust_ty(&arg.meta_type),
                        );
                    }

                    if self.is_return_interface_ty(field) {
                        let name = field.meta_type.name();
                        fn_scope.generic(&format!("T: {}", name));
                        fn_scope.ret(Type::new(&return_ty.replace(name, "T")));
                    } else {
                        fn_scope.ret(Type::new(&return_ty));
                    }

                    let block_str = build_block_str(&field, &field_name);
                    fn_scope.line(block_str);
                }
            }
            impl_str.push(scope.to_string());
        }

        let mut impl_scope = Scope::new();
        let struct_imp = impl_scope.new_impl(&struct_name.to_string());

        for field in &self.def.fields {
            let field_name = &field.name.to_snake_case();
            let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
            if is_return_primitive_ty(&field) {
                struct_scope.field(&field_name, &return_ty);
            }

            if implemented_fields.contains(&field_name) {
                continue;
            }

            let fn_scope = struct_imp.new_fn(&field_name);
            for arg in &field.arguments {
                fn_scope.arg(
                    &arg.name.to_snake_case(),
                    gql_value_ty_to_rust_ty(&arg.meta_type),
                );
            }

            fn_scope.arg_ref_self();
            fn_scope.set_async(true);

            if self.is_return_interface_ty(field) {
                let name = &field.meta_type.name();
                fn_scope.generic(&format!("T: {}", &name));
                fn_scope.ret(Type::new(&return_ty.replace(name, "T")));
            } else {
                fn_scope.ret(Type::new(&return_ty));
            }

            let block_str = build_block_str(&field, &field_name);
            fn_scope.line(block_str);
        }

        let impl_content = impl_str.join("\n");
        format!(
            "{}\n\n{}\n\n{}\n\n{}",
            use_gql_definitions(),
            struct_scope_base.to_string(),
            impl_content,
            impl_scope.to_string()
        )
    }
}

impl<'a> ObjectFile<'a> {
    fn is_return_interface_ty(&self, field: &GqlField) -> bool {
        let interface_names = self.interfaces_map.keys().collect::<Vec<_>>();
        interface_names.contains(&&field.meta_type.name().to_string())
    }
}

fn is_return_primitive_ty(field: &GqlField) -> bool {
    is_gql_primitive_ty(&field.meta_type.name())
}

fn build_block_str(field: &GqlField, name: &str) -> String {
    let block_str = if is_return_primitive_ty(&field) {
        format!("self.{}.clone()", &name)
    } else {
        "todo!()".to_string()
    };
    block_str
}
