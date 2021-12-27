use crate::{GqlField, Resolver, Schema, SelectionSetResolver};

use super::{input_value::__InputValue, introspection_type::__Type};

pub(crate) struct __Field<'a> {
    schema: &'a Schema,
    detail: GqlField,
}

impl<'a> __Field<'a> {
    pub fn new(schema: &'a Schema, field: GqlField) -> Self {
        __Field {
            schema,
            detail: field,
        }
    }

    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&String> {
        self.detail.description.as_ref()
    }

    async fn args(&'a self) -> Vec<__InputValue<'a>> {
        let mut result = Vec::new();

        for arg in &self.detail.arguments {
            let value = __InputValue::new(self.schema, arg);
            result.push(value);
        }
        result
    }

    async fn ty(&'a self) -> __Type<'a> {
        __Type::from_value_type(self.schema, &self.detail.meta_type)
    }

    async fn is_deprecated(&self) -> bool {
        self.detail.is_deprecated()
    }
}

#[async_trait::async_trait]
impl<'a> Resolver for __Field<'a> {
    async fn resolve_field(
        &self,
        ctx: &crate::FieldContext<'_>,
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

        if ctx.item.name == "isDeprecated" {
            let is_deprecated = self.is_deprecated().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(&is_deprecated, &ctx_selection_set)
                .await
                .map(Some);
        }
        Ok(None)
    }
}

#[async_trait::async_trait]
impl<'a> SelectionSetResolver for __Field<'a> {
    async fn resolve_selection_set(
        &self,
        ctx: &crate::SelectionSetContext<'_>,
    ) -> crate::ResolverResult<crate::GqlValue> {
        ctx.resolve_selection_parallelly(self).await
    }
}
