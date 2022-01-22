use std::collections::HashMap;

use codegen::{Scope, Type};
use heck::ToSnakeCase;
use rusty_gql::{GqlField, OperationType};

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct OperationModFile<'a> {
    pub operations: &'a HashMap<String, GqlField>,
    pub operation_type: OperationType,
    pub path: String,
}

impl<'a> FileDefinition for OperationModFile<'a> {
    fn name(&self) -> String {
        "mod.rs".to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut result = String::from("");

        for (operation_name, _) in self.operations.iter() {
            let file_name = operation_name.to_snake_case();
            result += format!("mod {};\n", file_name,).as_str();
        }

        result += "\n";
        result += &self.build_query_str();

        result
    }
}

impl<'a> OperationModFile<'a> {
    fn build_query_str(&self) -> String {
        let mut scope = Scope::new();
        let struct_name = self.operation_type.to_string();
        scope.new_struct(&struct_name).vis("pub");
        let imp = scope.new_impl(&struct_name);
        imp.r#macro("#[Resolver]");

        for (operation_name, method) in self.operations.iter() {
            let fn_scope = imp.new_fn(&operation_name);
            let mut args_str = String::from("");
            for arg in &method.arguments {
                fn_scope.arg(&arg.name, gql_value_ty_to_rust_ty(&arg.meta_type));
                args_str += format!("{},", &arg.name).as_str();
            }
            // remove last `,`
            args_str.pop();
            fn_scope.set_async(true);
            fn_scope.vis("pub");
            fn_scope.arg_ref_self();

            let return_ty = gql_value_ty_to_rust_ty(&method.meta_type);
            fn_scope.ret(Type::new(&return_ty));

            let file_name = operation_name.to_snake_case();
            fn_scope.line(format!(
                "{file_name}::{method}({args}).await",
                file_name = file_name,
                method = method.name,
                args = args_str
            ));
        }

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
