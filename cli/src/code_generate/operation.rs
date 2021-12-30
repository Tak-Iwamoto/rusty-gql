use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures_util::future::try_join_all;
use rusty_gql::{self, GqlField, OperationType};

use super::utils::{create_file, PathStr};

pub async fn build_operation_files(
    operations: &BTreeMap<String, GqlField>,
    operation_type: OperationType,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut field_names = Vec::new();
    let operation_str = match operation_type {
        OperationType::Query => "query",
        OperationType::Mutation => "mutation",
        OperationType::Subscription => "subscription",
    };

    for (_, field) in operations.iter() {
        let task = build_operation_file(field, operation_str);
        field_names.push(&field.name);
        futures.push(task);
    }
    let mod_file_str = build_mod_file_str(&operations, operation_type);
    let mod_file_path = PathStr::new(vec![operation_str, "mod"]).to_string();
    create_file(&mod_file_path, &mod_file_str).await?;

    let res = try_join_all(futures).await;
    res
}

fn build_mod_file_str(
    operations: &BTreeMap<String, GqlField>,
    operation_type: OperationType,
) -> String {
    let mut result = String::from("");

    for (file_name, _) in operations.iter() {
        result += format!("mod {file_name};\n", file_name = file_name,).as_str();
    }

    result += "\n";
    result += &build_query_str(operations, operation_type);

    result
}

fn build_query_str(
    operations: &BTreeMap<String, GqlField>,
    operation_type: OperationType,
) -> String {
    let mut scope = Scope::new();
    let struct_name = match operation_type {
        OperationType::Query => "Query",
        OperationType::Mutation => "Mutation",
        OperationType::Subscription => "Subscription",
    };
    scope.new_struct(struct_name).vis("pub");
    let imp = scope.new_impl(struct_name);

    for (operation_name, method) in operations.iter() {
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

async fn build_operation_file(field: &GqlField, operation_str: &str) -> Result<(), Error> {
    let path = PathStr::new(vec![operation_str, &field.name]).to_string();
    if tokio::fs::File::open(&path).await.is_err() {
        let content = build_field_str(&field);
        create_file(&path, &content).await?;
        Ok(())
    } else {
        Ok(())
    }
}

fn build_field_str(field: &GqlField) -> String {
    let mut scope = Scope::new();
    let fn_scope = scope.new_fn(field.name.as_str());

    for arg in &field.arguments {
        fn_scope.arg(arg.name.as_str(), arg.meta_type.to_rust_type_str());
    }
    fn_scope.vis("pub");
    scope.to_string()
}
