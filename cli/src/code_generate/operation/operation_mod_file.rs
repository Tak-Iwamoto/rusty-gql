use std::collections::BTreeMap;

use codegen::Scope;
use rusty_gql::{GqlField, OperationType};

use crate::code_generate::{build_file_path_str, FileStrategy};

pub struct OperationGqlModFile<'a> {
    pub operations: &'a BTreeMap<String, GqlField>,
    pub operation_type: OperationType,
    pub base_path: String,
}

impl<'a> FileStrategy for OperationGqlModFile<'a> {
    fn path(&self) -> String {
        let path = format!(
            "{}/{}",
            &self.operation_type.to_string().to_lowercase(),
            "mod"
        );
        build_file_path_str(&self.base_path, vec![&path])
    }

    fn content(&self) -> String {
        let mut result = String::from("");

        for (file_name, _) in self.operations.iter() {
            result += format!("mod {file_name};\n", file_name = file_name,).as_str();
        }

        result += "\n";
        result += &self.build_query_str();

        result
    }
}

impl<'a> OperationGqlModFile<'a> {
    fn build_query_str(&self) -> String {
        let mut scope = Scope::new();
        let struct_name = self.operation_type.to_string();
        scope.new_struct(&struct_name).vis("pub");
        let imp = scope.new_impl(&struct_name);

        for (operation_name, method) in self.operations.iter() {
            let f = imp.new_fn(&operation_name);
            let mut args_str = String::from("");
            for arg in &method.arguments {
                f.arg(arg.name.as_str(), arg.meta_type.to_rust_type_str());
                args_str += format!("{},", &arg.name).as_str();
            }

            f.line(format!(
                "{file_name}::{method}({args})",
                file_name = operation_name,
                method = method.name,
                args = args_str
            ));
        }

        scope.to_string()
    }
}
