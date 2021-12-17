use crate::Resolver;

pub(crate) struct QueryRoot<T> {
    query: T,
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for QueryRoot<T> {
    async fn resolve_field(
        &self,
        ctx: &crate::FieldContext<'_>,
    ) -> crate::ResolverResult<Option<crate::GqlValue>> {
        if ctx.item.name == "__schema" {
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);
            Ok(None)
        } else if ctx.item.name == "__type" {
            Ok(None)
        } else {
            self.query.resolve_field(ctx).await
        }
    }
}
