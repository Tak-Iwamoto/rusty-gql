use codegen::Scope;
use rusty_gql::GqlInterface;

use crate::code_generate::FileStrategy;

pub struct InterfaceFile<'a> {
    pub def: &'a GqlInterface,
}

impl<'a> FileStrategy for InterfaceFile<'a> {
    fn base_path(&self) -> String {
        "interface".to_string()
    }

    fn file_name(&self) -> String {
        self.def.name.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let trait_scope = scope.new_trait(self.def.name.as_str()).vis("pub");

        for field in &self.def.fields {
            trait_scope
                .new_fn(&field.name)
                .ret(field.meta_type.to_rust_type_str());
        }
        scope.to_string()
    }
}
