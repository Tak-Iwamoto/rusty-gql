use crate::{FieldContext, GqlValue, ResolverResult};

#[async_trait::async_trait]
pub trait CustomDirective {
    async fn call(&self, ctx: FieldContext<'_>) -> ResolverResult<Option<GqlValue>>;
}
