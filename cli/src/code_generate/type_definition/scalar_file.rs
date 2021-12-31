use codegen::Scope;
use rusty_gql::GqlScalar;

use crate::code_generate::{graphql_file_path, FileStrategy};

pub struct ScalarFile<'a> {
    pub def: &'a GqlScalar,
}

impl<'a> FileStrategy for ScalarFile<'a> {
    fn path(&self) -> String {
        graphql_file_path(vec!["scalar", &self.def.name])
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        scope.new_struct(self.def.name.as_str()).vis("pub");

        scope.to_string()
    }
}
