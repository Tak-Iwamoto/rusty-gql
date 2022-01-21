use std::collections::HashMap;

use codegen::Scope;
use rusty_gql::GqlInterface;

use crate::code_generate::{use_gql_definitions, FileDefinition};

pub struct InterfaceFile<'a> {
    pub file_name: &'a str,
    pub def: &'a GqlInterface,
    pub path: &'a str,
    pub interface_obj_map: &'a HashMap<String, Vec<String>>,
}

impl<'a> FileDefinition for InterfaceFile<'a> {
    fn name(&self) -> String {
        self.file_name.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let interface_scope = scope.new_enum(&self.def.name).vis("pub");
        interface_scope.derive("Union");

        if let Some(impl_objects) = self.interface_obj_map.get(&self.def.name) {
            for obj_name in impl_objects {
                interface_scope.new_variant(format!("{}({})", obj_name, obj_name).as_str());
            }
        }
        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
