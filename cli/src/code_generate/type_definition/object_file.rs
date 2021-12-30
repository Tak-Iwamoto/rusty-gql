use codegen::Scope;
use rusty_gql::GqlObject;

use super::TypeDefinitionFileStrategy;

pub struct ObjectFile<'a> {
    pub def: &'a GqlObject,
}

impl<'a> TypeDefinitionFileStrategy for ObjectFile<'a> {
    fn content(&self) -> String {
        let mut scope = Scope::new();
        let struct_scope = scope.new_struct(self.def.name.as_str()).vis("pub");

        for field in &self.def.fields {
            struct_scope.field(&field.name, field.meta_type.to_rust_type_str());
        }

        scope.to_string()
    }

    fn file_name(&self) -> String {
        self.def.name.to_string()
    }

    fn base_path(&self) -> String {
        "model".to_string()
    }
}
