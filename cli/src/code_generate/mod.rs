mod directive;
mod operation;
mod type_definition;
mod utils;

use std::io::Error;

use futures_util::future::try_join_all;
use rusty_gql::{build_schema, OperationType};
use utils::create_file;

use self::{
    operation::build_operation_files, type_definition::build_type_definition_files, utils::PathStr,
};

pub(crate) trait FileStrategy {
    fn content(&self) -> String;

    fn file_name(&self) -> String;

    fn base_path(&self) -> String;
}
pub(crate) async fn build_file<T: FileStrategy>(strategy: T) -> Result<(), Error> {
    let base_path = strategy.base_path();
    let file_name = strategy.file_name();
    let path = PathStr {
        paths: vec![base_path.as_str(), file_name.as_str()],
        base_path: None,
    }
    .to_string();
    if tokio::fs::File::open(&path).await.is_err() {
        let content = strategy.content();
        create_file(&path, &content).await?;
        Ok(())
    } else {
        Ok(())
    }
}

pub async fn build_graphql_schema(schema_doc: &str) -> Result<(), Error> {
    let schema = build_schema(schema_doc).unwrap();

    create_dirs().await?;
    create_mod_file().await?;

    let query_task = build_operation_files(&schema.queries, OperationType::Query);
    let mutation_task = build_operation_files(&schema.mutations, OperationType::Mutation);
    let subscription_task =
        build_operation_files(&schema.subscriptions, OperationType::Subscription);

    try_join_all(vec![query_task, mutation_task, subscription_task]).await?;

    let types_task = build_type_definition_files(&schema.type_definitions);
    types_task.await?;
    Ok(())
}

async fn create_mod_file() -> tokio::io::Result<()> {
    create_file("graphql/mod.rs", "mod input;\nmod interface;\nmod model;\nmod query;\nmod mutation;\nmod subscription;\nmod scalar;\n").await
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
