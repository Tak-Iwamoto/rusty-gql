use codegen::Scope;
use rusty_gql::GqlUnion;

use crate::code_generate::FileStrategy;

pub struct UnionFile<'a> {
    pub def: &'a GqlUnion,
}

impl<'a> FileStrategy for UnionFile<'a> {
    fn content(&self) -> String {
        let mut scope = Scope::new();
        let enum_scope = scope.new_enum(self.def.name.as_str()).vis("pub");

        for value in &self.def.types {
            enum_scope.new_variant(&value);
        }

        scope.to_string()
    }

    fn base_path(&self) -> String {
        "model".to_string()
    }

    fn file_name(&self) -> String {
        self.def.name.to_string()
    }
}
