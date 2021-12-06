mod utils;

use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures::future::try_join_all;
use rusty_gql::{self, build_schema, GqlField, GqlTypeDefinition};

use self::utils::{create_file, PathStr};

pub async fn gen_graphql_schema(schema_doc: &str) -> Result<(), Error> {
    let schema = build_schema(schema_doc).unwrap();

    create_dirs().await?;

    let query_task = gen_operation_files(&schema.queries, "query");
    let mutation_task = gen_operation_files(&schema.mutations, "mutation");
    let subscription_task = gen_operation_files(&schema.subscriptions, "subscription");

    try_join_all(vec![query_task, mutation_task, subscription_task]).await?;

    let types_task = gen_type_definition_files(&schema.type_definitions);
    types_task.await?;
    Ok(())
}

async fn gen_operation_files(
    operations: &BTreeMap<String, GqlField>,
    operation_str: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut field_names = Vec::new();
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
        create_file(&path, &gen_field_str(&field)).await?;
        Ok(())
    } else {
        Ok(())
    }
}

fn gen_field_str(field: &GqlField) -> String {
    let mut scope = Scope::new();
    let fn_scope = scope.new_fn(field.name.as_str());

    for arg in &field.arguments {
        fn_scope.arg(arg.name.as_str(), arg.meta_type.name());
    }
    fn_scope.vis("pub");
    scope.to_string()
}

async fn gen_type_definition_files(
    type_definitions: &BTreeMap<String, GqlTypeDefinition>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    for (_, type_def) in type_definitions.iter() {
        let task = gen_type_definition_file(type_def);
        futures.push(task);
    }
    let res = try_join_all(futures).await;
    res
}

async fn gen_type_definition_file(type_def: &GqlTypeDefinition) -> Result<(), Error> {
    let path =
        PathStr::new(vec![&type_def.to_string().to_lowercase(), type_def.name()]).to_string();
    if tokio::fs::File::open(&path).await.is_err() {
        create_file(&path, &gen_type_definition_str(&type_def)).await?;
        Ok(())
    } else {
        Ok(())
    }
}

fn gen_type_definition_str(type_def: &GqlTypeDefinition) -> String {
    let mut scope = Scope::new();
    let struct_scope = scope.new_struct(type_def.name());

    if let Some(fields) = type_def.fields() {
        for field in fields {
            struct_scope.field(&field.name, field.meta_type.name());
        }
    }

    scope.to_string()
}

async fn create_dirs() -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    // dirを作るときはcliのroot配下に作成される
    futures.push(tokio::fs::create_dir_all("graphql"));
    futures.push(tokio::fs::create_dir_all("graphql/query"));
    futures.push(tokio::fs::create_dir_all("graphql/mutation"));
    futures.push(tokio::fs::create_dir_all("graphql/subscription"));
    futures.push(tokio::fs::create_dir_all("graphql/inputobject"));
    futures.push(tokio::fs::create_dir_all("graphql/object"));
    futures.push(tokio::fs::create_dir_all("graphql/scalar"));
    futures.push(tokio::fs::create_dir_all("graphql/union"));
    futures.push(tokio::fs::create_dir_all("graphql/enum"));
    futures.push(tokio::fs::create_dir_all("graphql/interface"));
    futures.push(tokio::fs::create_dir_all("graphql/list"));
    let res = try_join_all(futures).await;
    res
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::graphql::gen_graphql_schema;

    #[tokio::test]
    async fn it_works() {
        let schema_doc = fs::read_to_string("../src/tests/github.graphql").unwrap();
        match gen_graphql_schema(&schema_doc).await {
            Ok(_) => println!("success"),
            Err(err) => println!("{}", err.to_string()),
        }
    }
}
