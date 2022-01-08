use codegen::{Scope, Type};
use rusty_gql::GqlField;

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct FieldFile<'a> {
    pub file_name: String,
    pub def: &'a GqlField,
    pub path: String,
    pub interface_names: &'a Vec<String>,
}

impl<'a> FileDefinition for FieldFile<'a> {
    fn name(&self) -> String {
        self.file_name.clone()
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

        let is_interface_return_ty = self
            .interface_names
            .contains(&self.def.meta_type.name().to_string());
        let return_ty = gql_value_ty_to_rust_ty(&self.def.meta_type);

        if is_interface_return_ty {
            let name = &self.def.meta_type.name();
            fn_scope.generic(&format!("T: {}", name));
            fn_scope.ret(Type::new(&return_ty.replace(name, "T")));
        } else {
            fn_scope.ret(Type::new(&return_ty));
        }

        fn_scope.vis("pub");
        fn_scope.set_async(true);
        fn_scope.line("todo!()");

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
