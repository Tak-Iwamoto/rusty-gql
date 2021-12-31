use codegen::Scope;
use rusty_gql::GqlField;

use crate::code_generate::FileStrategy;

pub struct FieldFile<'a> {
    pub def: &'a GqlField,
    pub path: String,
}

impl<'a> FileStrategy for FieldFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let fn_scope = scope.new_fn(self.def.name.as_str());

        for arg in &self.def.arguments {
            fn_scope.arg(arg.name.as_str(), arg.meta_type.to_rust_type_str());
        }
        fn_scope.vis("pub");
        fn_scope.line("todo!()");
        fn_scope.set_async(true);
        scope.to_string()
    }
}
