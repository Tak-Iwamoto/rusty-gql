use crate::{context::ExecutionContext, graphql_value::GraphQLValue, GraphQLResponse};
use async_trait::async_trait;

#[async_trait]
pub trait FieldResolver {
    async fn resolve_field(&self, ctx: &ExecutionContext) -> GraphQLResponse<GraphQLValue>;
}
