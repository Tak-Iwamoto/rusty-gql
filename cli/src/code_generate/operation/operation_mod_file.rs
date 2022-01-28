use std::collections::HashMap;

use codegen::{Scope, Type};
use heck::ToSnakeCase;
use rusty_gql::{FieldType, OperationType};

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct OperationModFile<'a> {
    pub operations: &'a HashMap<String, FieldType>,
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
        self.build_content()
    }
}

impl<'a> OperationModFile<'a> {
    fn build_content(&self) -> String {
        let mut scope = Scope::new();
        let struct_name = self.operation_type.to_string();
        let struct_scope = scope.new_struct(&struct_name).vis("pub");
        struct_scope.derive("Clone");
        let imp = scope.new_impl(&struct_name);
        imp.r#macro("#[GqlType]");

        let mut mod_str = "".to_string();
        for (operation_name, _) in self.operations.iter() {
            let filename = operation_name.to_snake_case();
            mod_str += format!("mod {};\n", filename,).as_str();
        }

        for (operation_name, method) in self.operations.iter() {
            let fn_scope = imp.new_fn(&operation_name);
            fn_scope.set_async(true);
            fn_scope.vis("pub");
            fn_scope.arg_ref_self();
            fn_scope.arg("ctx", "&Context<'_>");

            let mut args_str = String::from("");
            for arg in &method.arguments {
                fn_scope.arg(&arg.name, gql_value_ty_to_rust_ty(&arg.meta_type));
                args_str += format!("{},", &arg.name).as_str();
            }
            // remove last `,`
            args_str.pop();

            let return_ty = gql_value_ty_to_rust_ty(&method.meta_type);
            fn_scope.ret(Type::new(&return_ty));

            let filename = operation_name.to_snake_case();
            fn_scope.line(format!(
                "{filename}::{method}(&ctx,{args}).await",
                filename = filename,
                method = method.name,
                args = args_str
            ));
        }

        format!(
            "{}\n{}\n\n{}",
            use_gql_definitions(),
            mod_str,
            scope.to_string()
        )
    }
}
