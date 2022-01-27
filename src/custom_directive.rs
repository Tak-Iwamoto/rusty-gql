use std::collections::BTreeMap;

use crate::{Context, GqlValue, ResolveFut, ResolverResult};

#[async_trait::async_trait]
pub trait CustomDirective: Send + Sync {
    async fn resolve_field(
        &self,
        ctx: &Context<'_>,
        directive_args: &BTreeMap<String, GqlValue>,
        resolve_fut: ResolveFut<'_>,
    ) -> ResolverResult<Option<GqlValue>>;
}
