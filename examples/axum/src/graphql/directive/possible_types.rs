use crate::graphql::*;
use rusty_gql::*;
use std::collections::BTreeMap;

pub struct possibleTypes;

#[async_trait::async_trait]
impl CustomDirective for possibleTypes {
    async fn resolve_field(
        &self,
        ctx: &Context<'_>,
        directive_args: &BTreeMap<String, GqlValue>,
        resolve_fut: ResolveFut<'_>,
    ) -> ResolverResult<Option<GqlValue>> {
        todo!()
    }
}
