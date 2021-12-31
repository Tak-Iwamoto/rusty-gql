mod cargo_toml_file;
mod directive;
mod graphql_mod_file;
mod main_file;
mod operation;
mod type_definition;

use std::io::Error;

use futures_util::future::try_join_all;
use rusty_gql::{build_schema, OperationType};

use self::{
    directive::create_directive_files, graphql_mod_file::GqlModFile,
    operation::create_operation_files, type_definition::create_type_definition_files,
};
pub use cargo_toml_file::CargoTomlFile;
pub use main_file::MainFile;

use tokio::io::AsyncWriteExt;

pub(crate) trait FileStrategy {
    fn path(&self) -> String;

    fn content(&self) -> String;
}

pub(crate) async fn build_file<T: FileStrategy>(strategy: T) -> Result<(), Error> {
    let path = strategy.path();
    if tokio::fs::File::open(&path).await.is_err() {
        create_file(&path, &strategy.content()).await?;
        Ok(())
    } else {
        Ok(())
    }
}

pub(crate) fn graphql_file_path(paths: Vec<&str>) -> String {
    let file_path = paths.join("/");
    format!("graphql/{}.rs", file_path)
}

async fn create_file(path: &str, content: &str) -> Result<(), Error> {
    let mut file = tokio::fs::File::create(&path).await?;
    file.write(content.as_bytes()).await?;
    Ok(())
}

pub(crate) async fn create_gql_files(schema_documents: &[&str]) -> Result<(), Error> {
    let schema = build_schema(schema_documents).unwrap();

    create_root_dirs().await?;
    create_root_mod_file().await?;

    let query_task = create_operation_files(&schema.queries, OperationType::Query);
    let mutation_task = create_operation_files(&schema.mutations, OperationType::Mutation);
    let subscription_task =
        create_operation_files(&schema.subscriptions, OperationType::Subscription);

    try_join_all(vec![query_task, mutation_task, subscription_task]).await?;

    create_type_definition_files(&schema.type_definitions).await?;
    create_directive_files(&schema.directives).await?;
    Ok(())
}

async fn create_root_mod_file() -> tokio::io::Result<()> {
    let file_names = vec![
        "query".to_string(),
        "mutation".to_string(),
        "subscription".to_string(),
        "model".to_string(),
        "directive".to_string(),
        "scalar".to_string(),
        "input".to_string(),
        "interface".to_string(),
    ];
    build_file(GqlModFile {
        base_path: "".to_string(),
        file_names,
    })
    .await
}

async fn create_root_dirs() -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
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
