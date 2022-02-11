use crate::{Context, GqlValue, ResolveFut, ResolverResult};

#[async_trait::async_trait]
trait Middleware {
    async fn resolve(
        &self,
        ctx: &Context<'_>,
        resolve_fut: ResolveFut<'_>,
    ) -> ResolverResult<Option<GqlValue>>;
}
