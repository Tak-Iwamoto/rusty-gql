use crate::{
    error::GqlError,
    types::{__Schema, __Type},
    FieldContext, FieldResolver, GqlValue, ResolverResult, SelectionSetResolver,
};

pub struct QueryRoot<T> {
    pub query: T,
}

#[async_trait::async_trait]
impl<T: SelectionSetResolver> FieldResolver for QueryRoot<T> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        if ctx.item.name == "__schema" {
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);
            let schema_intro = __Schema::new(ctx.schema);
            return schema_intro
                .resolve_selection_set(&ctx_selection_set)
                .await
                .map(Some);
        } else if ctx.item.name == "__type" {
            let type_name = ctx.get_arg_value::<String>("name")?;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);
            let ty = ctx_selection_set
                .schema
                .type_definitions
                .get(&type_name)
                .map(|ty| __Type::from_type_definition(ctx_selection_set.schema, ty));
            match ty {
                Some(intro_ty) => intro_ty
                    .resolve_selection_set(&ctx_selection_set)
                    .await
                    .map(Some),
                None => Err(GqlError::new(format!("{} is not defined", type_name), None)),
            }
        } else {
            self.query.resolve_field(ctx).await
        }
    }
    fn type_name() -> String {
        "Query".to_string()
    }
}

#[async_trait::async_trait]
impl<T: SelectionSetResolver> SelectionSetResolver for QueryRoot<T> {
    async fn resolve_selection_set(
        &self,
        ctx: &crate::SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        self.query.resolve_selection_set(ctx).await
    }
}
