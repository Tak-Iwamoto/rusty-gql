use crate::{Resolver, Schema};

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

#[async_trait::async_trait]
impl<'a> Resolver for __Schema<'a> {
    async fn resolve_field(
        &self,
        ctx: &crate::FieldContext<'_>,
    ) -> crate::ResolverResult<Option<crate::GqlValue>> {
        Ok(None)
    }
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
