use crate::{
    CollectFields, FieldContext, FieldResolver, GqlValue, ResolverResult, SelectionSetContext,
    SelectionSetResolver, ID,
};

#[async_trait::async_trait]
impl FieldResolver for ID {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::String(self.0.to_string())))
    }
    fn type_name() -> String {
        "ID".to_string()
    }
}

impl CollectFields for ID {}

#[async_trait::async_trait]
impl SelectionSetResolver for ID {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::String(self.0.to_string()))
    }
}
