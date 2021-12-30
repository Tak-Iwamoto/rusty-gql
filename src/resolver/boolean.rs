use crate::{
    FieldContext, FieldResolver, GqlValue, ResolverResult, SelectionSetContext,
    SelectionSetResolver,
};

#[async_trait::async_trait]
impl FieldResolver for bool {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Boolean(*self)))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for bool {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Boolean(*self))
    }
}
