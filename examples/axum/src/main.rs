mod graphql;
mod starwars;

use rusty_gql::*;
use rusty_gql_axum::*;
use std::{net::SocketAddr, path::Path};

use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    AddExtensionLayer, Router,
};
use graphql::{Mutation, Query};

type ContainerType = Container<Query, Mutation, EmptySubscription>;

async fn gql_handler(container: Extension<ContainerType>, req: GqlRequest) -> GqlResponse {
    let result = execute(&container, req.0).await;
    GqlResponse::from(result)
}

async fn gql_playground() -> impl IntoResponse {
    response::Html(playground_html("/", None))
}

#[tokio::main]
async fn main() {
    let schema_docs = read_schemas(Path::new("./examples/axum/schemas")).unwrap();
    let schema_docs: Vec<&str> = schema_docs.iter().map(|s| &**s).collect();

    let container = Container::new(
        &schema_docs.as_slice(),
        Query,
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();
    let app = Router::new()
        .route("/", get(gql_playground).post(gql_handler))
        .layer(AddExtensionLayer::new(container));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
