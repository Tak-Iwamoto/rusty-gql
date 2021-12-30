use codegen::Scope;
use rusty_gql::GqlInputObject;

use crate::code_generate::FileStrategy;

pub struct InputObjectFile<'a> {
    pub def: &'a GqlInputObject,
}

impl<'a> FileStrategy for InputObjectFile<'a> {
    fn content(&self) -> String {
        let mut scope = Scope::new();
        let struct_scope = scope.new_struct(self.def.name.as_str()).vis("pub");

        for field in &self.def.fields {
            struct_scope.field(&field.name, field.meta_type.to_rust_type_str());
        }

        scope.to_string()
    }

    fn base_path(&self) -> String {
        "input".to_string()
    }

    fn file_name(&self) -> String {
        self.def.name.to_string()
    }
}
