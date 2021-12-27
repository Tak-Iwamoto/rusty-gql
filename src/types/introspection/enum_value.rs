use crate::{
    types::GqlEnumValue, FieldContext, GqlValue, Resolver, ResolverResult, Schema,
    SelectionSetResolver,
};

pub(crate) struct __EnumValue<'a> {
    schema: &'a Schema,
    detail: GqlEnumValue,
}

impl<'a> __EnumValue<'a> {
    pub fn new(schema: &'a Schema, value: &'a GqlEnumValue) -> Self {
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

    async fn is_deprecated(&self) -> bool {
        self.detail.is_deprecated()
    }
}

#[async_trait::async_trait]
impl<'a> Resolver for __EnumValue<'a> {
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
impl<'a> SelectionSetResolver for __EnumValue<'a> {
    async fn resolve_selection_set(
        &self,
        ctx: &crate::SelectionSetContext<'_>,
    ) -> crate::ResolverResult<crate::GqlValue> {
        ctx.resolve_selection_parallelly(self).await
    }
}
