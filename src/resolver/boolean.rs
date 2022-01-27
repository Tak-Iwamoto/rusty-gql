use crate::{
    Context, FieldResolver, GqlValue, ResolverResult, SelectionSetContext,
    SelectionSetResolver,
};

use super::CollectFields;

#[async_trait::async_trait]
impl FieldResolver for bool {
    async fn resolve_field(&self, _ctx: &Context<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Boolean(*self)))
    }
    fn type_name() -> String {
        "Boolean".to_string()
    }
}

impl CollectFields for bool {}

#[async_trait::async_trait]
impl SelectionSetResolver for bool {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Boolean(*self))
    }
}
