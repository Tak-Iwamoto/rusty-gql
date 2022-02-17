use anyhow::Result;
use app::build_app;
use async_recursion::async_recursion;
use exit_codes::ExitCode;
use std::process::Command;
use std::{path::Path, process};

use crate::code_generate::{create_gql_files, create_project_files};

mod app;
mod code_generate;
mod exit_codes;

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

fn gql_files_path(app_name: Option<&str>) -> String {
    match app_name {
        Some(path) => format!("{}/src/graphql", path),
        None => "src/graphql".to_string(),
    }
}

async fn create_graphql_files(app_name: Option<&str>) -> Result<(), std::io::Error> {
    let path = app_name
        .map(|name| format!("{}/schema", name))
        .unwrap_or_else(|| "schema".to_string());
    let schema_contents = visit_dirs(Path::new(&path)).await?;

    let schema_contents: Vec<&str> = schema_contents.iter().map(|s| &**s).collect();

    let gql_files_path = gql_files_path(app_name);
    create_gql_files(&schema_contents, &gql_files_path).await
}

fn run_fmt() {
    Command::new("cargo")
        .arg("fmt")
        .spawn()
        .expect("Failed to run cargo fmt.");
}

async fn run() -> Result<ExitCode> {
    let matches = build_app().get_matches();
    if matches.subcommand_matches("generate").is_some() {
        create_graphql_files(None).await?;
        run_fmt();
        return Ok(ExitCode::Success);
    }

    if let Some(new_matches) = matches.subcommand_matches("new") {
        if let Some(app_name) = new_matches.value_of("name") {
            create_project_files(app_name).await?;
            create_graphql_files(Some(app_name)).await?;
            println!("Successfully created the rusty-gql project!");
            return Ok(ExitCode::Success);
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
