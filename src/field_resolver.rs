use crate::{context::ExecutionContext, GqlValue, Response};
use async_trait::async_trait;

#[async_trait]
pub trait FieldResolver {
    async fn resolve_field(&self, ctx: &ExecutionContext) -> Response<GqlValue>;
}
