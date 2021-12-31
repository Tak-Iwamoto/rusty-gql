use codegen::Scope;
use rusty_gql::GqlUnion;

use crate::code_generate::{build_file_path_str, FileStrategy};

pub struct UnionFile<'a> {
    pub def: &'a GqlUnion,
    pub base_path: &'a str,
}

impl<'a> FileStrategy for UnionFile<'a> {
    fn path(&self) -> String {
        build_file_path_str(self.base_path, vec!["model", &self.def.name])
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
