use anyhow::Result;
use app::build_app;
use async_recursion::async_recursion;
use exit_codes::ExitCode;
use std::{path::Path, process};

use crate::code_generate::{create_gql_files, create_project_files};

mod app;
mod code_generate;
mod exit_codes;
mod mock;

#[async_recursion]
async fn visit_dirs(path: &Path) -> std::io::Result<Vec<String>> {
    let mut dir = tokio::fs::read_dir(path).await?;
    let mut schemas = Vec::new();
    while let Some(child) = dir.next_entry().await? {
        if child.metadata().await?.is_dir() {
            visit_dirs(&child.path()).await?;
        } else {
            let content = tokio::fs::read_to_string(child.path()).await?;
            schemas.push(content)
        }
    }

    Ok(schemas)
}

async fn generate_gql_files(app_name: Option<&str>) -> Result<(), std::io::Error> {
    let path = app_name
        .map(|name| format!("{}/schemas", name))
        .unwrap_or("./schemas".to_string());
    let files = visit_dirs(Path::new(&path)).await?;

    let files: Vec<&str> = files.iter().map(|s| &**s).collect();

    create_gql_files(&files).await
}

async fn run() -> Result<ExitCode> {
    let matches = build_app().get_matches();
    if matches.subcommand_matches("generate").is_some() {
        generate_gql_files(None).await?;
        return Ok(ExitCode::Success);
    }

    if let Some(new_matches) = matches.subcommand_matches("new") {
        if let Some(app_name) = new_matches.value_of("name") {
            create_project_files(app_name).await?;
            generate_gql_files(Some(app_name)).await?;
            println!("Successfully created the rusty-gql project!");
            return Ok(ExitCode::Success);
            // if let Some(server_lib) = new_matches.value_of("lib") {
            //     println!("server library: {:?}", server_lib);
            // }
        }
    }

    Ok(ExitCode::Success)
}

#[tokio::main]
async fn main() {
    let result = run().await;
    match result {
        Ok(code) => process::exit(code.into()),
        Err(err) => {
            eprintln!("rusty-gql: {:#}", err);
            process::exit(ExitCode::Failure.into())
        }
    }
}
