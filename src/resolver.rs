use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::{GraphQLResponse, executor::ExecutionContext, graphql_value::GraphQLValue, path::GraphQLPath};

// この型のvecを作成してfuture::joinに渡すことで並列に処理することができる。
pub type GraphQLFuture<'a> = BoxFuture<'a, GraphQLResponse<GraphQLValue>>;

// fieldごとにこのtraitを実装する
#[async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve(&self, context: &ExecutionContext) -> GraphQLResponse<GraphQLValue>;
}


pub(crate) struct ResolverInfo {
    field_name: String,
    return_type: GraphQLValue,
    parent_type: String,
    path: GraphQLPath,
}
