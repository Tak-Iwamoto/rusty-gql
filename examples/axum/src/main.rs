use rusty_gql::{execute, ArcContainer, Resolver};
use rusty_gql_axum::{GqlRequest, GqlResponse};
use std::net::SocketAddr;

use axum::{extract::Extension, routing::post, AddExtensionLayer, Router};

#[derive(Clone)]
struct Query;

#[rusty_gql::async_trait::async_trait]
impl Resolver for Query {
    async fn resolve_field(
        &self,
        ctx: &rusty_gql::FieldContext<'_>,
    ) -> rusty_gql::ResolverResult<Option<rusty_gql::GqlValue>> {
        Ok(None)
    }
}

#[derive(Clone)]
struct Mutation;

#[rusty_gql::async_trait::async_trait]
impl Resolver for Mutation {
    async fn resolve_field(
        &self,
        ctx: &rusty_gql::FieldContext<'_>,
    ) -> rusty_gql::ResolverResult<Option<rusty_gql::GqlValue>> {
        Ok(None)
    }
}

#[derive(Clone)]
struct Subscription;

#[rusty_gql::async_trait::async_trait]
impl Resolver for Subscription {
    async fn resolve_field(
        &self,
        ctx: &rusty_gql::FieldContext<'_>,
    ) -> rusty_gql::ResolverResult<Option<rusty_gql::GqlValue>> {
        Ok(None)
    }
}

type Container = ArcContainer<Query, Mutation, Subscription>;

async fn graphql_handler(container: Extension<Container>, req: GqlRequest) -> GqlResponse {
    let result = execute(&container, req.0).await;
    GqlResponse::from(result)
}

async fn test() -> &'static str {
    "test"
}

#[tokio::main]
async fn main() {
    let schema_doc = std::fs::read_to_string("./src/tests/starwars.graphql").unwrap();
    let container = ArcContainer::new(schema_doc.as_str(), Query, Mutation, Subscription);
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        // .route("/graphql", get(test))
        .layer(AddExtensionLayer::new(container));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
