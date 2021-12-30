use anyhow::{anyhow, Result};
use app::build_app;
use exit_codes::ExitCode;
use std::process;

use crate::code_generate::create_gql_files;

mod app;
mod code_generate;
mod exit_codes;

async fn run() -> Result<ExitCode> {
    let matches = build_app().get_matches();
    if matches.subcommand_matches("generate").is_some() {
        let schema_doc = std::fs::read_to_string("../tests/schemas/github.graphql").unwrap();
        create_gql_files(&schema_doc).await?;
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
