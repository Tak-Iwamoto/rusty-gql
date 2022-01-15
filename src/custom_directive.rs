use crate::{FieldContext, GqlValue, ResolveFut, ResolverResult};

#[async_trait::async_trait]
pub trait CustomDirective: Send + Sync {
    async fn call(
        &self,
        ctx: &FieldContext<'_>,
        resolve_fut: ResolveFut<'_>,
    ) -> ResolverResult<Option<GqlValue>>;
}
