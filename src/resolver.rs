use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::{context::ExecutionContext, path::GraphQLPath, GqlValue, Response};

// この型のvecを作成してfuture::joinに渡すことで並列に処理することができる。
pub type GraphQLFuture<'a> = BoxFuture<'a, Response<GqlValue>>;

// fieldごとにこのtraitを実装する
#[async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve(&self, context: &ExecutionContext) -> Response<GqlValue>;
}

pub(crate) struct ResolverInfo {
    field_name: String,
    return_type: GqlValue,
    parent_type: String,
    path: GraphQLPath,
}
