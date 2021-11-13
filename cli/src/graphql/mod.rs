use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures::future::try_join_all;
use rusty_gql::{self, build_schema, GqlField, GqlType, OperationType};
use tokio::io::AsyncWriteExt;

async fn generate_graphql_schema(schema_doc: &str) -> Result<(), Error> {
    let schema = build_schema(schema_doc).unwrap();

    let types = schema.type_map;

    let queries = schema.queries;
    let mutations = schema.mutations;
    let subscriptions = schema.subscriptions;

    create_dirs().await?;

    let query_task = generate_operations(&queries, OperationType::Query);
    let mutation_task = generate_operations(&mutations, OperationType::Mutation);
    let subscription_task = generate_operations(&subscriptions, OperationType::Subscription);

    try_join_all(vec![query_task, mutation_task, subscription_task]).await?;
    Ok(())
}

async fn generate_operations(
    operations: &BTreeMap<String, GqlField>,
    operation_type: OperationType,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    for (_, field) in operations.iter() {
        let task = generate_operation_file(field, &operation_type);
        futures.push(task);
    }
    let res = try_join_all(futures).await;
    res
}

async fn create_dirs() -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    // dirを作るときはcliのroot配下に作成される
    futures.push(tokio::fs::create_dir_all("./query"));
    futures.push(tokio::fs::create_dir_all("./mutation"));
    futures.push(tokio::fs::create_dir_all("./subscription"));
    futures.push(tokio::fs::create_dir_all("./model"));
    let res = try_join_all(futures).await;
    res
}

fn generate_field_str(field: &GqlField) -> String {
    let mut scope = Scope::new();
    let fn_scope = scope.new_fn(field.name.as_str());

    for arg in &field.arguments {
        fn_scope.arg(arg.name.as_str(), "String");
    }
    scope.to_string()
}

async fn generate_operation_file(
    field: &GqlField,
    operation_type: &OperationType,
) -> Result<(), Error> {
    let mut file =
        tokio::fs::File::create(format!("{}/{}.rs", operation_type.to_string(), field.name))
            .await?;
    file.write(generate_field_str(&field).as_bytes()).await?;
    Ok(())
}

fn generate_types(types_map: &BTreeMap<String, GqlType>) {
    for (key, gql_type) in types_map.iter() {
        let mut scope = Scope::new();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::graphql::generate_graphql_schema;

    #[tokio::test]
    async fn it_works() {
        let schema_doc = fs::read_to_string("../src/tests/github.graphql").unwrap();
        generate_graphql_schema(&schema_doc).await;
    }
}
