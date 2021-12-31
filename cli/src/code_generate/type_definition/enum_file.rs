use codegen::Scope;
use rusty_gql::GqlEnum;

use crate::code_generate::{build_file_path, FileStrategy};
pub struct EnumFile<'a> {
    pub def: &'a GqlEnum,
    pub base_path: &'a str,
}

impl<'a> FileStrategy for EnumFile<'a> {
    fn path(&self) -> String {
        build_file_path(self.base_path, vec!["model", &self.def.name])
    }
    fn content(&self) -> String {
        let mut scope = Scope::new();
        let enum_scope = scope.new_enum(self.def.name.as_str()).vis("pub");

        for value in &self.def.values {
            enum_scope.new_variant(&value.name);
        }

        scope.to_string()
    }
}
