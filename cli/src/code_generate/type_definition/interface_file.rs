use std::collections::HashMap;

use codegen::Scope;
use rusty_gql::InterfaceType;

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct InterfaceFile<'a> {
    pub filename: &'a str,
    pub def: &'a InterfaceType,
    pub path: &'a str,
    pub interface_obj_map: &'a HashMap<String, Vec<String>>,
}

impl<'a> FileDefinition for InterfaceFile<'a> {
    fn name(&self) -> String {
        self.filename.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let interface_name = &self.def.name;
        let interface_scope = scope.new_enum(interface_name).vis("pub");
        interface_scope.derive("GqlInterface");
        interface_scope.derive("Clone");

        if let Some(impl_objects) = self.interface_obj_map.get(interface_name) {
            for obj_name in impl_objects {
                interface_scope.new_variant(format!("{}({})", obj_name, obj_name).as_str());
            }
        }

        let mut impl_scope = Scope::new();
        let interface_impl = impl_scope.new_impl(interface_name);
        interface_impl.r#macro("#[GqlType(interface)]");

        for field in &self.def.fields {
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
            if let Some(impl_objects) = self.interface_obj_map.get(interface_name) {
                for obj_name in impl_objects {
                    let args = &field
                        .arguments
                        .iter()
                        .map(|arg| arg.name.clone())
                        .collect::<Vec<_>>()
                        .join(",");
                    fn_scope.line(format!(
                        "{interface_name}::{obj_name}(obj) => {{obj.{field_name}(&ctx, {args}).await}}",
                        interface_name = interface_name, obj_name = &obj_name, field_name = &field.name, args = args
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
}
