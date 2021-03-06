use crate::{
    resolve_selection_parallelly, CollectFields, FieldResolver, FieldType, GqlValue,
    ResolverResult, Schema, SelectionSetContext, SelectionSetResolver,
};

use super::{
    input_value::{__InputValue, build_input_value_introspection},
    introspection_type::__Type,
};

pub(crate) struct __Field<'a> {
    schema: &'a Schema,
    detail: FieldType,
}

impl<'a> __Field<'a> {
    pub fn new(schema: &'a Schema, field: FieldType) -> Self {
        __Field {
            schema,
            detail: field,
        }
    }

    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&str> {
        self.detail.description.as_deref()
    }

    async fn args(&'a self) -> Vec<__InputValue<'a>> {
        let mut result = Vec::new();

        for arg in &self.detail.arguments {
            let value = build_input_value_introspection(self.schema, arg);
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
impl<'a> FieldResolver for __Field<'a> {
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

        if ctx.item.name == "args" {
            let args = self.args().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(&args, &ctx_selection_set)
                .await
                .map(Some);
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
    fn type_name() -> String {
        "__Field".to_string()
    }
}

impl<'a> CollectFields for __Field<'a> {}

#[async_trait::async_trait]
impl<'a> SelectionSetResolver for __Field<'a> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        resolve_selection_parallelly(ctx, self).await
    }
}
