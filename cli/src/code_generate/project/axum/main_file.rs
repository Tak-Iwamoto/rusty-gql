use crate::code_generate::FileDefinition;
use codegen::Scope;

pub struct AxumMainFile<'a> {
    pub app_name: &'a str,
}

impl<'a> FileDefinition for AxumMainFile<'a> {
    fn name(&self) -> String {
        "main.rs".to_string()
    }

    fn path(&self) -> String {
        format!("{}/src/main.rs", self.app_name)
    }

    fn content(&self) -> String {
        main_file_content()
    }
}

fn main_file_content() -> String {
    let contents = vec![
        axum_import_str(),
        axum_gql_handler(),
        axum_gql_playground(),
        axum_main_function(),
    ];
    contents.join("\n\n")
}

fn axum_import_str() -> String {
    let statements = vec![
        "mod graphql;",
        "use rusty_gql::*;",
        "use rusty_gql_axum::*;",
        "use std::{net::SocketAddr, path::Path};",
        "use axum::{routing::get, AddExtensionLayer, Router};",
        "use graphql::Query;",
        "type ContainerType = Container<Query, EmptyMutation, EmptySubscription>;",
    ];
    statements.join("\n")
}

fn axum_gql_handler() -> String {
    let mut scope = Scope::new();
    let f = scope.new_fn("gql_handler");
    f.set_async(true);
    f.arg("container", "axum::extract::Extension<ContainerType>");
    f.arg("req", "GqlRequest");
    f.ret("GqlResponse");
    f.line("let result = execute(&container, req.0).await;");
    f.line("GqlResponse::from(result)");

    scope.to_string()
}

fn axum_gql_playground() -> String {
    let mut scope = Scope::new();
    let f = scope.new_fn("gql_playground");
    f.set_async(true);
    f.ret("impl axum::response::IntoResponse");
    f.line("axum::response::Html(playground_html(\"/\", None))");

    scope.to_string()
}

fn axum_main_function() -> String {
    let mut scope = Scope::new();
    let f = scope.new_fn("main");
    f.set_async(true);
    f.line("let schema_docs = read_schemas(Path::new(\"./src/schema\")).unwrap();");
    f.line("let schema_docs: Vec<&str> = schema_docs.iter().map(|s| &**s).collect();");
    f.line("let container = Container::new(&schema_docs.as_slice(), Query, EmptyMutation, EmptySubscription, Default::default(),).unwrap();");
    f.line("let app = Router::new().route(\"/graphiql\", get(gql_playground)).route(\"/\", get(gql_handler).post(gql_handler)).layer(AddExtensionLayer::new(container));");
    f.line("let addr = SocketAddr::from(([127, 0, 0, 1], 3000));");
    f.line("axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();");

    format!("#[tokio::main]\n{}", scope.to_string())
}
