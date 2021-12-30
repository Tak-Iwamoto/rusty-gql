use crate::{
    FieldContext, FieldResolver, GqlValue, ResolverResult, SelectionSetContext,
    SelectionSetResolver,
};

#[async_trait::async_trait]
impl<T: FieldResolver> FieldResolver for Option<T> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        match self {
            Some(resolver) => resolver.resolve_field(ctx).await,
            None => Ok(Some(GqlValue::Null)),
        }
    }
}

#[async_trait::async_trait]
impl<T: SelectionSetResolver> SelectionSetResolver for Option<T> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        match self {
            Some(resolver) => resolver.resolve_selection_set(ctx).await,
            None => Ok(GqlValue::Null),
        }
    }
}
