use codegen::Scope;
use rusty_gql::GqlEnum;

use crate::code_generate::FileStrategy;
pub struct EnumFile<'a> {
    pub def: &'a GqlEnum,
}

impl<'a> FileStrategy for EnumFile<'a> {
    fn base_path(&self) -> String {
        "model".to_string()
    }

    fn file_name(&self) -> String {
        self.def.name.to_string()
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
