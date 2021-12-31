use codegen::Scope;
use rusty_gql::GqlInputObject;

use crate::code_generate::FileDefinition;

pub struct InputObjectFile<'a> {
    pub def: &'a GqlInputObject,
    pub path: &'a str,
}

impl<'a> FileDefinition for InputObjectFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let struct_scope = scope.new_struct(self.def.name.as_str()).vis("pub");

        for field in &self.def.fields {
            struct_scope.field(&field.name, field.meta_type.to_rust_type_str());
        }

        scope.to_string()
    }
}
