use codegen::Scope;
use rusty_gql::GqlUnion;

use crate::code_generate::{graphql_file_path, FileStrategy};

pub struct UnionFile<'a> {
    pub def: &'a GqlUnion,
}

impl<'a> FileStrategy for UnionFile<'a> {
    fn path(&self) -> String {
        graphql_file_path(vec!["model", &self.def.name])
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let enum_scope = scope.new_enum(self.def.name.as_str()).vis("pub");

        for value in &self.def.types {
            enum_scope.new_variant(&value);
        }

        scope.to_string()
    }
}
