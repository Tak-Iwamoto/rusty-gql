use codegen::Scope;
use rusty_gql::GqlScalar;

use crate::code_generate::{build_file_path, FileStrategy};

pub struct ScalarFile<'a> {
    pub def: &'a GqlScalar,
    pub base_path: &'a str,
}

impl<'a> FileStrategy for ScalarFile<'a> {
    fn path(&self) -> String {
        build_file_path(self.base_path, vec!["scalar", &self.def.name])
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        scope.new_struct(self.def.name.as_str()).vis("pub");

        scope.to_string()
    }
}
