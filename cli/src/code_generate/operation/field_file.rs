use codegen::Scope;
use rusty_gql::GqlField;

use crate::code_generate::FileStrategy;

pub struct FieldFile<'a> {
    pub def: &'a GqlField,
    pub base_path: String,
}

impl<'a> FileStrategy for FieldFile<'a> {
    fn base_path(&self) -> String {
        self.base_path.to_string()
    }

    fn file_name(&self) -> String {
        self.def.name.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let fn_scope = scope.new_fn(self.def.name.as_str());

        for arg in &self.def.arguments {
            fn_scope.arg(arg.name.as_str(), arg.meta_type.to_rust_type_str());
        }
        fn_scope.vis("pub");
        scope.to_string()
    }
}
