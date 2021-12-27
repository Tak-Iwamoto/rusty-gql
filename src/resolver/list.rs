use crate::{
    FieldContext, GqlValue, Resolver, ResolverResult, SelectionSetContext, SelectionSetResolver,
};

#[async_trait::async_trait]
impl<T: Resolver, const N: usize> Resolver for [T; N] {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut result = Vec::new();
        for value in self {
            if let Some(v) = value.resolve_field(ctx).await? {
                result.push(v);
            }
        }
        Ok(Some(GqlValue::List(result)))
    }
}

#[async_trait::async_trait]
impl<T: SelectionSetResolver, const N: usize> SelectionSetResolver for [T; N] {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut result = Vec::new();
        for value in self {
            let v = value.resolve_selection_set(ctx).await?;
            result.push(v);
        }
        Ok(GqlValue::List(result))
    }
}
