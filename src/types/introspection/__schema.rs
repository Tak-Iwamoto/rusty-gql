use crate::{Resolver, Schema};

// type __Schema {
//     types: [__Type!]!
//     queryType: __Type!
//     mutationType: __Type
//     subscriptionType: __Type
//     directives: [__Directive!]!
//   }
pub(crate) struct __Schema<'a> {
    schema_def: &'a Schema,
}

#[async_trait::async_trait]
impl<'a> Resolver for __Schema<'a> {
    async fn resolve_field(
        &self,
        ctx: &crate::FieldContext<'_>,
    ) -> crate::ResolverResult<Option<crate::GqlValue>> {
        Ok(None)
    }
}
