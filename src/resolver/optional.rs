use crate::{
    CollectFields, Context, FieldResolver, GqlValue, ResolverResult, SelectionSetContext,
    SelectionSetResolver,
};

#[async_trait::async_trait]
impl<T: FieldResolver> FieldResolver for Option<T> {
    async fn resolve_field(&self, ctx: &Context<'_>) -> ResolverResult<Option<GqlValue>> {
        match self {
            Some(resolver) => resolver.resolve_field(ctx).await,
            None => Ok(Some(GqlValue::Null)),
        }
    }
    fn type_name() -> String {
        T::type_name()
    }
}

impl<T: FieldResolver> CollectFields for Option<T> {}

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
