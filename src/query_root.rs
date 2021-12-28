use crate::{
    error::GqlError,
    types::{__Schema, __Type},
    GqlValue, Resolver, ResolverResult, SelectionSetResolver,
};

pub struct QueryRoot<T> {
    pub query: T,
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for QueryRoot<T> {
    async fn resolve_field(
        &self,
        ctx: &crate::FieldContext<'_>,
    ) -> ResolverResult<Option<crate::GqlValue>> {
        if ctx.item.name == "__schema" {
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);
            let schema_intro = __Schema::new(ctx.schema);
            return schema_intro
                .resolve_selection_set(&ctx_selection_set)
                .await
                .map(Some);
        } else if ctx.item.name == "__type" {
            match &ctx.get_arg_value("name") {
                Some(value) => {
                    let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);
                    if let GqlValue::String(v) = value {
                        let ty =
                            ctx_selection_set.schema.type_definitions.get(v).map(|ty| {
                                __Type::from_type_definition(ctx_selection_set.schema, ty)
                            });
                        match ty {
                            Some(intro_ty) => SelectionSetResolver::resolve_selection_set(
                                &intro_ty,
                                &ctx_selection_set,
                            )
                            .await
                            .map(Some),
                            None => Err(GqlError::new(format!("{} is not defined", v), None)),
                        }
                    } else {
                        Ok(None)
                    }
                }
                None => {
                    return Ok(None);
                }
            }
        } else {
            self.query.resolve_field(ctx).await
        }
    }
}
