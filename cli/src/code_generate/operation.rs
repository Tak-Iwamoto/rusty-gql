use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures::future::try_join_all;
use rusty_gql::{self, GqlField, OperationType};

use super::utils::{create_file, PathStr};

pub async fn gen_operation_files(
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
        let task = gen_operation_file(field, operation_str);
        field_names.push(&field.name);
        futures.push(task);
    }
    let mod_file_str = build_mod_file_str(&operations);
    let mod_file_path = PathStr::new(vec![operation_str, "mod"]).to_string();
    create_file(&&mod_file_path, &mod_file_str).await?;

    let res = try_join_all(futures).await;
    res
}

fn build_mod_file_str(operations: &BTreeMap<String, GqlField>) -> String {
    let mut result = String::from("");

    for (file_name, method) in operations.iter() {
        // pub use field::GqlField;
        result += format!(
            "mod {file_name};\npub use {file_name}::{method};\n\n",
            file_name = file_name,
            method = method.name
        )
        .as_str();
    }

    result
}

async fn gen_operation_file(field: &GqlField, operation_str: &str) -> Result<(), Error> {
    let path = PathStr::new(vec![operation_str, &field.name]).to_string();
    if tokio::fs::File::open(&path).await.is_err() {
        let content = gen_field_str(&field);
        create_file(&path, &content).await?;
        Ok(())
    } else {
        Ok(())
    }
}

fn gen_field_str(field: &GqlField) -> String {
    let mut scope = Scope::new();
    let fn_scope = scope.new_fn(field.name.as_str());

    for arg in &field.arguments {
        fn_scope.arg(arg.name.as_str(), arg.meta_type.to_rust_type());
    }
    fn_scope.vis("pub");
    scope.to_string()
}
