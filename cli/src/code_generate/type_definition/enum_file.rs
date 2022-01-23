use codegen::Scope;
use rusty_gql::GqlEnum;

use crate::code_generate::{use_gql_definitions, FileDefinition};
pub struct EnumFile<'a> {
    pub filename: &'a str,
    pub def: &'a GqlEnum,
    pub path: &'a str,
}

impl<'a> FileDefinition for EnumFile<'a> {
    fn name(&self) -> String {
        self.filename.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let enum_scope = scope.new_enum(&self.def.name).vis("pub");
        enum_scope.derive("Enum");
        enum_scope.derive("Copy");
        enum_scope.derive("Clone");
        enum_scope.derive("Eq");
        enum_scope.derive("PartialEq");

        for value in &self.def.values {
            enum_scope.new_variant(&value.name);
        }

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
