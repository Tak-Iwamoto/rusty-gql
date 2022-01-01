use crate::code_generate::FileDefinition;

pub struct CargoTomlFile<'a> {
    pub app_name: &'a str,
}

impl<'a> FileDefinition for CargoTomlFile<'a> {
    fn path(&self) -> String {
        format!("{}/Cargo.toml", self.app_name)
    }

    fn content(&self) -> String {
        cargo_toml_content(self.app_name)
    }
}

fn cargo_toml_content(app_name: &str) -> String {
    r#"[package]
name = "APP_NAME"
version = "0.1.0"
edition = "2021"

[dependencies]
async-trait = "0.1.52"
axum = {version = "0.4.2", features = ["headers"]}
hyper = "0.14.16"
rusty-gql = {registry = "ktra", version = "0.0.3"}
rusty-gql-axum = {registry = "ktra", version = "0.0.1"}
tokio = {version = "1.12.0", features = ["full"]}"#
        .replace("APP_NAME", app_name)
}
