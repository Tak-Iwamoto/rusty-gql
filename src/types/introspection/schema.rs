use crate::*;
use rusty_gql_macro::GqlType;

use crate::Schema;

use super::{directive::__Directive, introspection_type::__Type};

pub struct __Schema<'a> {
    detail: &'a Schema,
}

pub fn build_schema_introspection(schema: &Schema) -> __Schema<'_> {
    __Schema { detail: schema }
}

#[allow(non_snake_case)]
#[GqlType(internal)]
impl<'a> __Schema<'a> {
    async fn types(&self) -> Vec<__Type<'a>> {
        let mut result = Vec::new();
        for def in self.detail.type_definitions.values() {
            let ty = __Type::from_type_definition(self.detail, def);
            result.push(ty);
        }

        result
    }

    async fn queryType(&self) -> __Type<'a> {
        match self
            .detail
            .type_definitions
            .get(&self.detail.query_type_name)
        {
            Some(query) => __Type::from_type_definition(self.detail, query),
            None => panic!("Query is not defined."),
        }
    }

    async fn mutationType(&self) -> Option<__Type<'a>> {
        self.detail
            .type_definitions
            .get(&self.detail.mutation_type_name)
            .map(|mutation| __Type::from_type_definition(self.detail, mutation))
    }

    async fn subscriptionType(&self) -> Option<__Type<'a>> {
        self.detail
            .type_definitions
            .get(&self.detail.subscription_type_name)
            .map(|subscription| __Type::from_type_definition(self.detail, subscription))
    }

    async fn directives(&self) -> Vec<__Directive<'a>> {
        let mut result = Vec::new();

        for dir in self.detail.directives.values() {
            let directive = __Directive::new(self.detail, dir);
            result.push(directive);
        }
        result
    }
}
