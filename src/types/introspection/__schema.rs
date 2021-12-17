use crate::{Resolver, Schema};

// type __Schema {
//     types: [__Type!]!
//     queryType: __Type!
//     mutationType: __Type
//     subscriptionType: __Type
//     directives: [__Directive!]!
//   }
pub(crate) struct __Schema {
    schema_def: &Schema,
}

impl Resolver for __Schema {
    async fn resolve_field(
        &self,
        ctx: &crate::FieldContext<'_>,
    ) -> crate::ResolverResult<Option<crate::GqlValue>> {
    }
}
