use async_trait::async_trait;

use crate::{graphql_value::GraphQLValue, GraphQLResponse};

#[async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve(&self) -> GraphQLResponse<GraphQLValue>;
}
