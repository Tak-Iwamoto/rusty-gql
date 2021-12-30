use codegen::Scope;
use rusty_gql::GqlScalar;

use crate::code_generate::FileStrategy;

pub struct ScalarFile<'a> {
    pub def: &'a GqlScalar,
}

impl<'a> FileStrategy for ScalarFile<'a> {
    fn base_path(&self) -> String {
        "scalar".to_string()
    }

    fn file_name(&self) -> String {
        self.def.name.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        scope.new_struct(self.def.name.as_str()).vis("pub");

        scope.to_string()
    }
}
