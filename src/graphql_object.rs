use crate::{graphql_value::GraphQLValue, GraphQLResponse};
use async_trait::async_trait;

#[async_trait]
pub trait GraphQLObject {
    async fn resolve_field(&self) -> GraphQLResponse<Option<GraphQLValue>>;
}

pub(crate) struct Query<T> {
    pub(crate) parent_type: T,
}
