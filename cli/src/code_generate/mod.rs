mod directive;
mod graphql_mod_file;
mod operation;
mod project;
mod type_definition;

use std::io::Error;

use futures_util::future::try_join_all;
use rusty_gql::{build_schema, OperationType};

use self::{
    directive::create_directive_files, graphql_mod_file::GqlModFile,
    operation::create_operation_files, type_definition::create_type_definition_files,
};

pub use project::create_project_files;
use tokio::io::AsyncWriteExt;

pub(crate) trait FileStrategy {
    fn path(&self) -> String;

    fn content(&self) -> String;
}

pub(crate) async fn create_file<T: FileStrategy>(strategy: T) -> Result<(), Error> {
    let path = strategy.path();
    if tokio::fs::File::open(&path).await.is_err() {
        let mut file = tokio::fs::File::create(&path).await?;
        file.write(strategy.content().as_bytes()).await?;
        Ok(())
    } else {
        Ok(())
    }
}

pub(crate) fn build_file_path_str(base_path: &str, paths: Vec<&str>) -> String {
    let file_path = paths.join("/");
    format!("{}/{}.rs", base_path, file_path)
}

pub(crate) async fn create_gql_files(
    schema_documents: &[&str],
    base_path: Option<&str>,
) -> Result<(), Error> {
    let schema = build_schema(schema_documents).unwrap();

    let gql_base_path = graphql_path(base_path);
    create_root_dirs(&gql_base_path).await?;
    create_root_mod_file(&gql_base_path).await?;

    let query_task = create_operation_files(&schema.queries, OperationType::Query, &gql_base_path);
    let mutation_task =
        create_operation_files(&schema.mutations, OperationType::Mutation, &gql_base_path);
    let subscription_task = create_operation_files(
        &schema.subscriptions,
        OperationType::Subscription,
        &gql_base_path,
    );

    try_join_all(vec![query_task, mutation_task, subscription_task]).await?;

    create_type_definition_files(&schema.type_definitions, &gql_base_path).await?;
    create_directive_files(&schema.directives, &gql_base_path).await?;
    Ok(())
}

fn graphql_path(base_path: Option<&str>) -> String {
    match base_path {
        Some(path) => format!("{}/graphql", path),
        None => "graphql".to_string(),
    }
}

fn gql_file_types() -> Vec<String> {
    vec![
        "query".to_string(),
        "mutation".to_string(),
        "subscription".to_string(),
        "model".to_string(),
        "directive".to_string(),
        "scalar".to_string(),
        "input".to_string(),
        "interface".to_string(),
    ]
}
async fn create_root_mod_file(base_path: &str) -> tokio::io::Result<()> {
    let file_names = gql_file_types();
    create_file(GqlModFile {
        path: base_path,
        file_names,
    })
    .await
}

async fn create_root_dirs(base_path: &str) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    for name in gql_file_types() {
        futures.push(tokio::fs::create_dir_all(format!("{}/{}", base_path, name)));
    }
    let res = try_join_all(futures).await;
    res
}
