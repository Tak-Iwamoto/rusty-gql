use crate::{
    CollectFields, FieldContext, FieldResolver, GqlValue, ResolverResult, SelectionSetContext,
    SelectionSetResolver,
};

#[async_trait::async_trait]
impl FieldResolver for str {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::String(self.to_string())))
    }
    fn type_name() -> String {
        "String".to_string()
    }
}

impl CollectFields for str {}

#[async_trait::async_trait]
impl SelectionSetResolver for str {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::String(self.to_string()))
    }
}

#[async_trait::async_trait]
impl FieldResolver for &str {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::String(self.to_string())))
    }
    fn type_name() -> String {
        "String".to_string()
    }
}

impl CollectFields for &str {}

#[async_trait::async_trait]
impl SelectionSetResolver for &str {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::String(self.to_string()))
    }
}

#[async_trait::async_trait]
impl FieldResolver for String {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::String(self.clone())))
    }
    fn type_name() -> String {
        "String".to_string()
    }
}

impl CollectFields for String {}

#[async_trait::async_trait]
impl SelectionSetResolver for String {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::String(self.clone()))
    }
}
