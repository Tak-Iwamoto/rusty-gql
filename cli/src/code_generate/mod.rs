mod operation;
mod type_definition;
mod utils;

use std::io::Error;

use futures::future::try_join_all;
use rusty_gql::{build_schema, OperationType};

use self::{operation::gen_operation_files, type_definition::gen_type_definition_files};

pub async fn gen_graphql_schema(schema_doc: &str) -> Result<(), Error> {
    let schema = build_schema(schema_doc).unwrap();

    create_dirs().await?;

    let query_task = gen_operation_files(&schema.queries, OperationType::Query);
    let mutation_task = gen_operation_files(&schema.mutations, OperationType::Mutation);
    let subscription_task = gen_operation_files(&schema.subscriptions, OperationType::Subscription);

    try_join_all(vec![query_task, mutation_task, subscription_task]).await?;

    let types_task = gen_type_definition_files(&schema.type_definitions);
    types_task.await?;
    Ok(())
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

    use crate::code_generate::gen_graphql_schema;

    #[tokio::test]
    async fn it_works() {
        let schema_doc = fs::read_to_string("../src/tests/github.graphql").unwrap();
        match gen_graphql_schema(&schema_doc).await {
            Ok(_) => println!("success"),
            Err(err) => println!("{}", err.to_string()),
        }
    }
}
