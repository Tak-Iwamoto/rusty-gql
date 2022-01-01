use codegen::{Scope, Type};
use rusty_gql::GqlField;

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct FieldFile<'a> {
    pub def: &'a GqlField,
    pub path: String,
}

impl<'a> FileDefinition for FieldFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let fn_scope = scope.new_fn(self.def.name.as_str());

        for arg in &self.def.arguments {
            fn_scope.arg(arg.name.as_str(), gql_value_ty_to_rust_ty(&arg.meta_type));
        }
        fn_scope.vis("pub");
        fn_scope.set_async(true);
        fn_scope.line("todo!()");
        fn_scope.ret(Type::new(&self.def.meta_type.name()));

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
