use rusty_gql::{execute, playground_html, Container, EmptyMutation, EmptySubscription, Resolver};
use rusty_gql_axum::{GqlRequest, GqlResponse};
use std::net::SocketAddr;

use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    AddExtensionLayer, Router,
};

#[derive(Clone)]
struct Query;

#[Resolver]
impl Query {}

type ContainerType = Container<Query, EmptyMutation, EmptySubscription>;

async fn gql_handler(container: Extension<ContainerType>, req: GqlRequest) -> GqlResponse {
    let result = execute(&container, req.0).await;
    GqlResponse::from(result)
}

async fn gql_playground() -> impl IntoResponse {
    response::Html(playground_html("/", None))
}

#[tokio::main]
async fn main() {
    let schema_doc = std::fs::read_to_string("./tests/schemas/starwars.graphql").unwrap();

    let container = Container::new(
        &vec![schema_doc.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();
    let app = Router::new()
        .route("/", get(gql_playground).post(gql_handler))
        // .route("/graphql", get(test))
        .layer(AddExtensionLayer::new(container));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
