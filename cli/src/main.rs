use anyhow::{anyhow, Result};
use app::build_app;
use async_recursion::async_recursion;
use exit_codes::ExitCode;
use std::{path::Path, process};

use crate::code_generate::create_gql_files;

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

async fn run() -> Result<ExitCode> {
    let matches = build_app().get_matches();
    if matches.subcommand_matches("generate").is_some() {
        let path = std::env::current_dir().unwrap();
        println!("starting dir: {}", path.display());
        let files = visit_dirs(Path::new("./tests/schemas")).await?;

        let files: Vec<&str> = files.iter().map(|s| &**s).collect();

        create_gql_files(&files).await?;
        return Ok(ExitCode::Success);
    }

    if let Some(new_matches) = matches.subcommand_matches("new") {
        if let Some(app_name) = new_matches.value_of("name") {
            let output = process::Command::new("cargo")
                .arg("new")
                .arg(app_name)
                .output();
            if let Some(server_lib) = new_matches.value_of("lib") {
                println!("server library: {:?}", server_lib);
            }
            match output {
                Ok(_) => {
                    println!("Successfully created the rusty-gql project!");
                    return Ok(ExitCode::Success);
                }
                Err(_) => return Err(anyhow!("Failed to init rusty-gql project")),
            }
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
