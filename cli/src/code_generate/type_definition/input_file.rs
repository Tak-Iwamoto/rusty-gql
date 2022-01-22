use codegen::Scope;
use rusty_gql::GqlInputObject;

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct InputObjectFile<'a> {
    pub file_name: &'a str,
    pub def: &'a GqlInputObject,
    pub path: &'a str,
}

impl<'a> FileDefinition for InputObjectFile<'a> {
    fn name(&self) -> String {
        self.file_name.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let struct_scope = scope.new_struct(&self.def.name).vis("pub");
        struct_scope.derive("InputObject");

        for field in &self.def.fields {
            struct_scope.field(&field.name, gql_value_ty_to_rust_ty(&field.meta_type));
        }

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
