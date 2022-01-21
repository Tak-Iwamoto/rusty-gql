use codegen::Scope;
use rusty_gql::GqlScalar;

use crate::code_generate::{use_gql_definitions, FileDefinition};

pub struct ScalarFile<'a> {
    pub file_name: &'a str,
    pub def: &'a GqlScalar,
    pub path: &'a str,
}

impl<'a> FileDefinition for ScalarFile<'a> {
    fn name(&self) -> String {
        self.file_name.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let scalar_scope = scope.new_struct(&self.def.name).vis("pub");
        scalar_scope.derive("Scalar");

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
