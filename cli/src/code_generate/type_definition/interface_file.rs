use codegen::{Scope, Type};
use heck::ToSnakeCase;
use rusty_gql::GqlInterface;

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct InterfaceFile<'a> {
    pub def: &'a GqlInterface,
    pub path: &'a str,
    pub interface_names: &'a Vec<String>,
}

impl<'a> FileDefinition for InterfaceFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let trait_scope = scope
            .new_trait(format!("{}: Send + Sync", self.def.name).as_str())
            .vis("pub");

        for field in &self.def.fields {
            let field_name = &field.name.to_snake_case();
            let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
            let is_interface_return_ty = self
                .interface_names
                .contains(&field.meta_type.name().to_string());
            if is_interface_return_ty {
                let f = trait_scope.new_fn(&field_name);
                f.set_async(true);
                f.arg_ref_self();
                f.generic(&format!("T: {}", &field.meta_type.name()));
                f.ret(Type::new("T"));
                for arg in &field.arguments {
                    f.arg(
                        &arg.name.to_snake_case(),
                        gql_value_ty_to_rust_ty(&arg.meta_type),
                    );
                }
            } else {
                let f = trait_scope.new_fn(&field_name);
                f.set_async(true);
                f.arg_ref_self();
                f.ret(Type::new(&return_ty));
                for arg in &field.arguments {
                    f.arg(
                        &arg.name.to_snake_case(),
                        gql_value_ty_to_rust_ty(&arg.meta_type),
                    );
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
