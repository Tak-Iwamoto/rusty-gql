use std::collections::BTreeMap;

use codegen::{Scope, Type};
use heck::ToSnakeCase;
use rusty_gql::{GqlInterface, GqlObject};

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
        let mut impl_scope = Scope::new();
        let struct_name = &self.def.name;
        let struct_scope = struct_scope_base
            .new_struct(&struct_name.to_string())
            .vis("pub");
        let imp = impl_scope.new_impl(&struct_name.to_string());

        let mut implemented_fields = Vec::new();
        let mut impl_str = Vec::new();
        for interface in &self.def.implements_interfaces {
            let mut scope = Scope::new();
            scope.new_struct(&struct_name);
            let impl_interface = scope.new_impl(&struct_name);
            impl_interface.impl_trait(interface);
            impl_interface.r#macro("#[async_trait::async_trait]");

            if let Some(gql_interface) = self.interfaces_map.get(interface) {
                for field in &gql_interface.fields {
                    let field_name = &field.name.to_snake_case();
                    let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
                    let f = impl_interface.new_fn(&field_name);
                    f.set_async(true);
                    implemented_fields.push(field_name.to_string());
                    let mut args_str = String::from("");
                    for arg in &field.arguments {
                        f.arg(
                            &arg.name.to_snake_case(),
                            gql_value_ty_to_rust_ty(&arg.meta_type),
                        );
                        args_str += format!("{},", &arg.name.to_snake_case()).as_str();
                    }
                    // remove last `,`
                    args_str.pop();

                    let interface_names = self.interfaces_map.keys().collect::<Vec<_>>();
                    let is_interface_return_ty =
                        interface_names.contains(&&field.meta_type.name().to_string());
                    if is_interface_return_ty {
                        f.generic(&format!("T: {}", &field.meta_type.name()));
                        f.ret(Type::new("T"));
                    } else {
                        f.ret(Type::new(&return_ty));
                    }
                    f.arg_ref_self();

                    let is_primitive_ty = is_gql_primitive_ty(&field.meta_type.name());
                    let block_str = if is_primitive_ty {
                        format!("self.{}.clone()", &field_name)
                    } else {
                        "todo!()".to_string()
                    };
                    f.line(block_str);
                }
            }
            impl_str.push(scope.to_string());
        }

        for field in &self.def.fields {
            let field_name = &field.name.to_snake_case();
            let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
            let is_primitive_ty = is_gql_primitive_ty(&field.meta_type.name());
            if is_primitive_ty {
                struct_scope.field(&field_name, &return_ty);
            }

            if implemented_fields.contains(&field_name) {
                continue;
            }

            let f = imp.new_fn(&field_name);
            let mut args_str = String::from("");
            for arg in &field.arguments {
                f.arg(
                    &arg.name.to_snake_case(),
                    gql_value_ty_to_rust_ty(&arg.meta_type),
                );
                args_str += format!("{},", &arg.name.to_snake_case()).as_str();
            }
            // remove last `,`
            args_str.pop();

            f.set_async(true);

            let interface_names = self.interfaces_map.keys().collect::<Vec<_>>();
            let is_interface_return_ty =
                interface_names.contains(&&field.meta_type.name().to_string());
            if is_interface_return_ty {
                f.generic(&format!("T: {}", &field.meta_type.name()));
                f.ret(Type::new("T"));
            } else {
                f.ret(Type::new(&return_ty));
            }
            f.arg_ref_self();

            let block_str = if is_primitive_ty {
                format!("self.{}.clone()", &field_name)
            } else {
                "todo!()".to_string()
            };
            f.line(block_str);
        }

        let impl_content = impl_str.join("\n");
        format!(
            "{}\n\n{}\n{}\n{}",
            use_gql_definitions(),
            struct_scope_base.to_string(),
            impl_content,
            impl_scope.to_string()
        )
    }
}
