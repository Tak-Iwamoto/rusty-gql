use codegen::Scope;
use rusty_gql::GqlScalar;

use crate::code_generate::FileDefinition;

pub struct ScalarFile<'a> {
    pub def: &'a GqlScalar,
    pub path: &'a str,
}

impl<'a> FileDefinition for ScalarFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        scope.new_struct(self.def.name.as_str()).vis("pub");

        scope.to_string()
    }
}
