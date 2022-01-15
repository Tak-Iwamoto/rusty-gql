use crate::{FieldContext, ResolverResult, Value};

#[async_trait::async_trait]
pub trait CustomDirective: Send + Sync {
    async fn call(&self, ctx: FieldContext<'_>) -> ResolverResult<Option<Value>>;
}
