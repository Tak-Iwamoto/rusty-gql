use crate::{executor::ExecutionContext, graphql_value::GraphQLValue, GraphQLResponse};
use async_trait::async_trait;

#[async_trait]
pub trait ObjectResolver {
    async fn resolve_field(&self, ctx: &ExecutionContext) -> GraphQLResponse<GraphQLValue>;
}
