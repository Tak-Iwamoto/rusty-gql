use crate::{FieldContext, ResolverResult, Value};

#[async_trait::async_trait]
pub trait SchemaDirective {
    async fn call(&self, ctx: FieldContext<'_>) -> ResolverResult<Option<Value>>;
}

#[async_trait::async_trait]
pub trait QueryDirective {
    async fn call(&self, ctx: FieldContext<'_>) -> ResolverResult<Option<Value>>;
}
