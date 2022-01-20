mod directive;
mod mod_file;
mod operation;
mod project;
mod root_mod_file;
mod type_definition;
mod util;

use std::io::Error;

use futures_util::future::try_join_all;
use rusty_gql::{build_schema, OperationType};

use self::{
    directive::create_directive_files, operation::create_operation_files,
    root_mod_file::RootModFile, type_definition::create_type_definition_files,
    util::get_interface_impl_object_map,
};

pub use project::create_project_files;
use tokio::io::AsyncWriteExt;

pub(crate) trait FileDefinition {
    fn name(&self) -> String;

    fn path(&self) -> String;

    fn content(&self) -> String;
}

pub(crate) async fn create_file<T: FileDefinition>(file_def: T) -> Result<(), Error> {
    let path = file_def.path();
    if tokio::fs::File::open(&path).await.is_err() {
        let mut file = tokio::fs::File::create(&path).await?;
        file.write(file_def.content().as_bytes()).await?;
        Ok(())
    } else {
        if file_def.name() == "mod.rs".to_string() {
            let mut file = tokio::fs::File::create(&path).await?;
            file.write(file_def.content().as_bytes()).await?;
            Ok(())
        } else {
            Ok(())
        }
    }
}

pub(crate) fn path_str(paths: Vec<&str>, is_file: bool) -> String {
    if is_file {
        let path_str = paths.join("/");
        format!("{}.rs", path_str)
    } else {
        paths.join("/")
    }
}

pub(crate) async fn create_gql_files(schema_documents: &[&str], path: &str) -> Result<(), Error> {
    let schema = match build_schema(schema_documents, Default::default()) {
        Ok(v) => v,
        Err(err) => return Err(Error::new(std::io::ErrorKind::InvalidInput, err.message)),
    };

    create_root_dirs(path).await?;
    create_root_mod_file(path).await?;

    let query_task = create_operation_files(&schema.queries, OperationType::Query, path);
    let mutation_task = create_operation_files(&schema.mutations, OperationType::Mutation, path);
    let subscription_task =
        create_operation_files(&schema.subscriptions, OperationType::Subscription, path);

    try_join_all(vec![query_task, mutation_task, subscription_task]).await?;

    let interface_obj_maps = get_interface_impl_object_map(&schema.type_definitions);
    create_type_definition_files(&schema, path, &interface_obj_maps).await?;
    create_directive_files(&schema.directives, path).await?;
    Ok(())
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
    ]
}
async fn create_root_mod_file(path: &str) -> tokio::io::Result<()> {
    let file_names = gql_file_types();
    create_file(RootModFile { path, file_names }).await
}

async fn create_root_dirs(path: &str) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    for name in gql_file_types() {
        futures.push(tokio::fs::create_dir_all(format!("{}/{}", path, name)));
    }
    try_join_all(futures).await
}

pub(crate) fn use_gql_definitions() -> &'static str {
    r#"use crate::graphql::*;
use rusty_gql::*;"#
}
