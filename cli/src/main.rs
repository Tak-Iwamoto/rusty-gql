use std::process;

use anyhow::Result;
use app::build_app;
use exit_codes::ExitCode;

use crate::code_generate::create_gql_files;

mod app;
mod code_generate;
mod exit_codes;

async fn run() -> Result<ExitCode> {
    let matches = build_app().get_matches();
    if matches.subcommand_matches("gen").is_some() {
        let schema_doc = std::fs::read_to_string("../tests/schemas/github.graphql").unwrap();
        create_gql_files(&schema_doc).await?;
        return Ok(ExitCode::Success);
    }

    if let Some(new_matches) = matches.subcommand_matches("new") {
        if let Some(app_name) = new_matches.value_of("name") {
            println!("app_name: {:?}", app_name);
        }

        if let Some(server_lib) = new_matches.value_of("lib") {
            println!("server library: {:?}", server_lib);
        }
        return Ok(ExitCode::Success);
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
