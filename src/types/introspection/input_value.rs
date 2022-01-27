use crate::{
    resolve_selection_parallelly, CollectFields, FieldResolver, GqlValue, InputValueType,
    ResolverResult, Schema, SelectionSetContext, SelectionSetResolver,
};

use super::introspection_type::__Type;

pub struct __InputValue<'a> {
    schema: &'a Schema,
    detail: InputValueType,
}
pub fn build_input_value_introspection<'a>(
    schema: &'a Schema,
    value: &'a InputValueType,
) -> __InputValue<'a> {
    __InputValue {
        schema,
        detail: value.clone(),
    }
}

impl<'a> __InputValue<'a> {
    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&str> {
        self.detail.description.as_deref()
    }

    async fn ty(&'a self) -> __Type<'a> {
        __Type::from_value_type(self.schema, &self.detail.meta_type)
    }

    async fn default_value(&self) -> Option<String> {
        match &self.detail.default_value {
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }
}

#[async_trait::async_trait]
impl<'a> FieldResolver for __InputValue<'a> {
    async fn resolve_field(
        &self,
        ctx: &crate::Context<'_>,
    ) -> crate::ResolverResult<Option<crate::GqlValue>> {
        if ctx.item.name == "name" {
            let name = self.name().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(name, &ctx_selection_set)
                .await
                .map(Some);
        }

        if ctx.item.name == "description" {
            let desc = self.description().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match desc {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }

        if ctx.item.name == "type" {
            let ty = self.ty().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(&ty, &ctx_selection_set)
                .await
                .map(Some);
        }

        if ctx.item.name == "defaultValue" {
            let is_deprecated = self.default_value().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(&is_deprecated, &ctx_selection_set)
                .await
                .map(Some);
        }
        Ok(None)
    }
    fn type_name() -> String {
        "__InputValue".to_string()
    }
}

impl<'a> CollectFields for __InputValue<'a> {}

#[async_trait::async_trait]
impl<'a> SelectionSetResolver for __InputValue<'a> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        resolve_selection_parallelly(ctx, self).await
    }
}
