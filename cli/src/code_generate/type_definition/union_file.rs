use codegen::Scope;
use rusty_gql::GqlUnion;

use crate::code_generate::{use_gql_definitions, FileDefinition};

pub struct UnionFile<'a> {
    pub filename: &'a str,
    pub def: &'a GqlUnion,
    pub path: &'a str,
}

impl<'a> FileDefinition for UnionFile<'a> {
    fn name(&self) -> String {
        self.filename.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let union_scope = scope.new_enum(&self.def.name).vis("pub");
        union_scope.derive("Union");

        for value in &self.def.types {
            union_scope.new_variant(&value).tuple(&value);
        }

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
