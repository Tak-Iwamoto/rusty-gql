mod axum;
mod example_schema_file;

use std::io::Error;

pub use example_schema_file::StarWarsSchemaFile;
use futures_util::future::try_join_all;

use self::axum::{AxumCargoTomlFile, AxumMainFile};

use super::create_file;

pub async fn create_project_files(app_name: &str) -> Result<(), Error> {
    try_join_all(vec![
        tokio::fs::create_dir_all(format!("{}/src", app_name).as_str()),
        tokio::fs::create_dir_all(format!("{}/schema", app_name).as_str()),
    ])
    .await?;
    create_main_file(app_name).await?;
    create_cargo_toml(app_name).await?;
    create_example_gql_schema(app_name).await
}

async fn create_main_file(app_name: &str) -> Result<(), Error> {
    create_file(AxumMainFile { app_name }).await
}

async fn create_cargo_toml(app_name: &str) -> Result<(), Error> {
    create_file(AxumCargoTomlFile { app_name }).await
}

async fn create_example_gql_schema(app_name: &str) -> Result<(), Error> {
    create_file(StarWarsSchemaFile { app_name }).await
}
