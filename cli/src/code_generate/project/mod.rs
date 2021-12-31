mod cargo_toml_file;
mod example_schema_file;
mod main_file;

use std::io::Error;

pub use cargo_toml_file::CargoTomlFile;
pub use example_schema_file::StarWarsSchemaFile;
use futures_util::future::try_join_all;
pub use main_file::MainFile;

use super::build_file;

pub async fn create_project_files(app_name: &str) -> Result<(), Error> {
    try_join_all(vec![
        tokio::fs::create_dir_all(format!("{}/src", app_name).as_str()),
        tokio::fs::create_dir_all(format!("{}/schemas", app_name).as_str()),
    ])
    .await?;
    create_main_file(app_name).await?;
    create_cargo_toml(app_name).await?;
    create_example_gql_schema(app_name).await
}

async fn create_main_file(app_name: &str) -> Result<(), Error> {
    build_file(MainFile { app_name }).await
}

async fn create_cargo_toml(app_name: &str) -> Result<(), Error> {
    build_file(CargoTomlFile { app_name }).await
}

async fn create_example_gql_schema(app_name: &str) -> Result<(), Error> {
    build_file(StarWarsSchemaFile { app_name }).await
}
