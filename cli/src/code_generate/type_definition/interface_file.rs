use codegen::{Scope, Type};
use rusty_gql::GqlInterface;

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct InterfaceFile<'a> {
    pub file_name: &'a str,
    pub def: &'a GqlInterface,
    pub path: &'a str,
    pub interface_names: &'a Vec<String>,
}

impl<'a> FileDefinition for InterfaceFile<'a> {
    fn name(&self) -> String {
        self.file_name.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let trait_scope = scope
            .new_trait(format!("{}: Send + Sync", self.def.name).as_str())
            .vis("pub");

        for field in &self.def.fields {
            let field_name = &field.name;
            let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
            let is_interface_return_ty = self
                .interface_names
                .contains(&field.meta_type.name().to_string());
            if is_interface_return_ty {
                let fn_scope = trait_scope.new_fn(&field_name);
                fn_scope.set_async(true);
                fn_scope.arg_ref_self();

                let name = &field.meta_type.name();
                fn_scope.generic(&format!("T: {}", &name));
                fn_scope.ret(Type::new(&return_ty.replace(name, "T")));
                for arg in &field.arguments {
                    fn_scope.arg(&arg.name, gql_value_ty_to_rust_ty(&arg.meta_type));
                }
            } else {
                let fn_scope = trait_scope.new_fn(&field_name);
                fn_scope.set_async(true);
                fn_scope.arg_ref_self();
                fn_scope.ret(Type::new(&return_ty));
                for arg in &field.arguments {
                    fn_scope.arg(&arg.name, gql_value_ty_to_rust_ty(&arg.meta_type));
                }
            }
        }
        format!(
            "{}\n\n#[async_trait::async_trait]\n{}",
            use_gql_definitions(),
            scope.to_string()
        )
    }
}
