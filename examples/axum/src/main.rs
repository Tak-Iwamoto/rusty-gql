use rusty_gql::{execute, ArcContainer, Object, Resolver};
use rusty_gql_axum::{GqlRequest, GqlResponse};
use std::net::SocketAddr;

use axum::{extract::Extension, handler::post, AddExtensionLayer, Router};

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

#[tokio::main]
async fn main() {
    let schema_doc = std::fs::read_to_string("./src/tests/starwars.graphql").unwrap();
    let query = Query {};
    let mutation = Mutation {};
    let subscription = Subscription {};
    let container = ArcContainer::new(schema_doc.as_str(), query, mutation, subscription);
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .layer(AddExtensionLayer::new(container));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
