use codegen::Scope;
use rusty_gql::GqlEnum;

use crate::code_generate::FileDefinition;
pub struct EnumFile<'a> {
    pub def: &'a GqlEnum,
    pub path: &'a str,
}

impl<'a> FileDefinition for EnumFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
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
