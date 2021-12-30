mod directive;
mod operation;
mod type_definition;

use std::io::Error;

use futures_util::future::try_join_all;
use rusty_gql::{build_schema, OperationType};

use self::{
    directive::create_directive_files, operation::create_operation_files,
    type_definition::create_type_definition_files,
};
use tokio::io::AsyncWriteExt;

pub(crate) trait FileStrategy {
    fn content(&self) -> String;

    fn file_name(&self) -> String;

    fn base_path(&self) -> String;
}

pub(crate) async fn build_file<T: FileStrategy>(strategy: T) -> Result<(), Error> {
    let path = format!(
        "graphql/{}/{}.rs",
        strategy.base_path(),
        strategy.file_name()
    );
    if tokio::fs::File::open(&path).await.is_err() {
        create_file(&path, &strategy.content()).await?;
        Ok(())
    } else {
        Ok(())
    }
}

async fn create_file(path: &str, content: &str) -> Result<(), Error> {
    let mut file = tokio::fs::File::create(&path).await?;
    file.write(content.as_bytes()).await?;
    Ok(())
}

pub(crate) async fn create_gql_files(schema_doc: &str) -> Result<(), Error> {
    let schema = build_schema(schema_doc).unwrap();

    create_dirs().await?;
    create_mod_file().await?;

    let query_task = create_operation_files(&schema.queries, OperationType::Query);
    let mutation_task = create_operation_files(&schema.mutations, OperationType::Mutation);
    let subscription_task =
        create_operation_files(&schema.subscriptions, OperationType::Subscription);

    try_join_all(vec![query_task, mutation_task, subscription_task]).await?;

    create_type_definition_files(&schema.type_definitions).await?;
    create_directive_files(&schema.directives).await?;
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
    futures.push(tokio::fs::create_dir_all("graphql/directive"));
    let res = try_join_all(futures).await;
    res
}
