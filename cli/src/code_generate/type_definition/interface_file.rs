use codegen::Scope;
use rusty_gql::GqlInterface;

use crate::code_generate::FileDefinition;

pub struct InterfaceFile<'a> {
    pub def: &'a GqlInterface,
    pub path: &'a str,
}

impl<'a> FileDefinition for InterfaceFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
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
