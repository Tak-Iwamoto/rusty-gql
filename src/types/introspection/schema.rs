use crate::{
    FieldContext, GqlValue, Resolver, ResolverResult, Schema, SelectionSetContext,
    SelectionSetResolver,
};

use super::{directive::__Directive, introspection_type::__Type};

// type __Schema {
//     types: [__Type!]!
//     queryType: __Type!
//     mutationType: __Type
//     subscriptionType: __Type
//     directives: [__Directive!]!
//   }
pub(crate) struct __Schema<'a> {
    detail: &'a Schema,
}

impl<'a> __Schema<'a> {
    async fn types(&self) -> Vec<__Type<'a>> {
        let mut result = Vec::new();
        for (_, def) in &self.detail.type_definitions {
            let ty = __Type::from_type_definition(self.detail, def);
            result.push(ty);
        }

        result
    }

    async fn query_type(&self) -> __Type<'a> {
        match self.detail.type_definitions.get("Query") {
            Some(query) => __Type::from_type_definition(self.detail, query),
            None => panic!("Query is not defined."),
        }
    }

    async fn mutation_type(&self) -> Option<__Type<'a>> {
        match self.detail.type_definitions.get("Mutation") {
            Some(mutation) => Some(__Type::from_type_definition(self.detail, mutation)),
            None => None,
        }
    }

    async fn subscription_type(&self) -> Option<__Type<'a>> {
        match self.detail.type_definitions.get("Subscription") {
            Some(subscription) => Some(__Type::from_type_definition(self.detail, subscription)),
            None => None,
        }
    }

    async fn directives(&self) -> Vec<__Directive<'a>> {
        let mut result = Vec::new();

        for (_, dir) in &self.detail.directives {
            let directive = __Directive::new(self.detail, dir);
            result.push(directive);
        }
        result
    }
}

#[async_trait::async_trait]
impl<'a> Resolver for __Schema<'a> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        if ctx.item.name == "types" {
            let types = self.types().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(&types, &ctx_selection_set)
                .await
                .map(Some);
        }
        if ctx.item.name == "queryType" {
            let ty = self.query_type().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(&ty, &ctx_selection_set)
                .await
                .map(Some);
        }
        if ctx.item.name == "mutationType" {
            let ty = self.mutation_type().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match ty {
                Some(mutation_ty) => {
                    return SelectionSetResolver::resolve_selection_set(
                        &mutation_ty,
                        &ctx_selection_set,
                    )
                    .await
                    .map(Some);
                }
                None => {
                    return Ok(None);
                }
            }
        }
        if ctx.item.name == "subscriptionType" {
            let ty = self.subscription_type().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match ty {
                Some(subscription_ty) => {
                    return SelectionSetResolver::resolve_selection_set(
                        &subscription_ty,
                        &ctx_selection_set,
                    )
                    .await
                    .map(Some);
                }
                None => {
                    return Ok(None);
                }
            }
        }
        if ctx.item.name == "directives" {
            let directives = self.directives().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(&directives, &ctx_selection_set)
                .await
                .map(Some);
        }

        Ok(None)
    }
}

#[async_trait::async_trait]
impl<'a> SelectionSetResolver for __Schema<'a> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        ctx.resolve_selection_parallelly(self).await
    }
}
