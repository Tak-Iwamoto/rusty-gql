use codegen::{Scope, Type};
use rusty_gql::GqlField;

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct FieldFile<'a> {
    pub filename: String,
    pub def: &'a GqlField,
    pub path: String,
}

impl<'a> FileDefinition for FieldFile<'a> {
    fn name(&self) -> String {
        self.filename.clone()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let fn_scope = scope.new_fn(&self.def.name);

        for arg in &self.def.arguments {
            fn_scope.arg(&arg.name, gql_value_ty_to_rust_ty(&arg.meta_type));
        }

        let return_ty = gql_value_ty_to_rust_ty(&self.def.meta_type);

        fn_scope.ret(Type::new(&return_ty));

        fn_scope.vis("pub");
        fn_scope.set_async(true);
        fn_scope.line("todo!()");

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
