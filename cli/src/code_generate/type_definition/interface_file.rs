use codegen::Scope;
use rusty_gql::GqlInterface;

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct InterfaceFile<'a> {
    pub def: &'a GqlInterface,
    pub path: &'a str,
    pub interface_names: &'a Vec<String>,
}

impl<'a> FileDefinition for InterfaceFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let trait_scope = scope
            .new_trait(format!("{}: Send + Sync", self.def.name).as_str())
            .vis("pub");

        for field in &self.def.fields {
            let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
            let return_name = if self
                .interface_names
                .contains(&field.meta_type.name().to_string())
            {
                format!("dyn {}", return_ty)
            } else {
                return_ty
            };
            trait_scope
                .new_fn(&field.name)
                .set_async(true)
                .arg_ref_self()
                .ret(return_name);
        }
        format!(
            "{}\n\n#[async_trait::async_trait]\n{}",
            use_gql_definitions(),
            scope.to_string()
        )
    }
}
