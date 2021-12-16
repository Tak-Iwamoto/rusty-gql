mod operation;
mod type_definition;
mod directive;
mod utils;

use std::io::Error;

use futures_util::future::try_join_all;
use rusty_gql::{build_schema, OperationType};

use self::{operation::build_operation_files, type_definition::build_type_definition_files};

pub async fn build_graphql_schema(schema_doc: &str) -> Result<(), Error> {
    let schema = build_schema(schema_doc).unwrap();

    create_dirs().await?;

    let query_task = build_operation_files(&schema.queries, OperationType::Query);
    let mutation_task = build_operation_files(&schema.mutations, OperationType::Mutation);
    let subscription_task = build_operation_files(&schema.subscriptions, OperationType::Subscription);

    try_join_all(vec![query_task, mutation_task, subscription_task]).await?;

    let types_task = build_type_definition_files(&schema.type_definitions);
    types_task.await?;
    Ok(())
}

async fn create_dirs() -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    futures.push(tokio::fs::create_dir_all("graphql"));
    futures.push(tokio::fs::create_dir_all("graphql/query"));
    futures.push(tokio::fs::create_dir_all("graphql/mutation"));
    futures.push(tokio::fs::create_dir_all("graphql/subscription"));
    futures.push(tokio::fs::create_dir_all("graphql/model"));
    futures.push(tokio::fs::create_dir_all("graphql/scalar"));
    futures.push(tokio::fs::create_dir_all("graphql/interface"));
    futures.push(tokio::fs::create_dir_all("graphql/input"));
    let res = try_join_all(futures).await;
    res
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::code_generate::build_graphql_schema;

    #[tokio::test]
    async fn it_works() {
        let schema_doc = fs::read_to_string("../src/tests/github.graphql").unwrap();
        match build_graphql_schema(&schema_doc).await {
            Ok(_) => println!("success"),
            Err(err) => println!("{}", err.to_string()),
        }
    }
}
