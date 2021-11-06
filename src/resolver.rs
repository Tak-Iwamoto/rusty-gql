use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::{executor::ExecutionContext, graphql_value::GraphQLValue, GraphQLResponse};

// この型のvecを作成してfuture::joinに渡すことで並列に処理することができる。
pub type GraphQLFuture<'a> = BoxFuture<'a, GraphQLResponse<GraphQLValue>>;

// fieldごとにこのtraitを実装する
#[async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve(&self, context: &ExecutionContext) -> GraphQLResponse<GraphQLValue>;
}

pub struct GraphQLPath {
    pub prev: Option<Box<GraphQLPath>>,
    pub key: String,
    pub typename: String,
}

pub(crate) struct ResolverInfo {
    field_name: String,
    return_type: GraphQLValue,
    parent_type: String,
    path: GraphQLPath,
}
