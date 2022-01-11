mod boolean;
mod list;
mod number;
mod object;
mod optional;
mod string;

use async_trait::async_trait;

use crate::{
    context::{FieldContext, SelectionSetContext},
    GqlValue, ResolverResult,
};

#[async_trait]
pub trait SelectionSetResolver: FieldResolver {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue>;
}

#[async_trait]
pub trait FieldResolver: Send + Sync {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>>;
    fn type_name() -> String;
}

#[async_trait::async_trait]
impl<T: FieldResolver> FieldResolver for &T {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        T::resolve_field(*self, ctx).await
    }

    fn type_name() -> String {
        T::type_name()
    }
}
