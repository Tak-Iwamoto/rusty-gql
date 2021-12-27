use crate::{
    FieldContext, GqlArgument, GqlValue, Resolver, ResolverResult, Schema, SelectionSetResolver,
};

use super::introspection_type::__Type;

pub(crate) struct __InputValue<'a> {
    schema: &'a Schema,
    detail: GqlArgument,
}

impl<'a> __InputValue<'a> {
    pub fn new(schema: &'a Schema, value: &'a GqlArgument) -> Self {
        Self {
            schema,
            detail: value.clone(),
        }
    }

    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&String> {
        self.detail.description.as_ref()
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
impl<'a> Resolver for __InputValue<'a> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
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
            let value = self.default_value().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match value {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(&v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }
        Ok(None)
    }
}

#[async_trait::async_trait]
impl<'a> SelectionSetResolver for __InputValue<'a> {
    async fn resolve_selection_set(
        &self,
        ctx: &crate::SelectionSetContext<'_>,
    ) -> crate::ResolverResult<crate::GqlValue> {
        ctx.resolve_selection_parallelly(self).await
    }
}
