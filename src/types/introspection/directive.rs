use graphql_parser::schema::DirectiveLocation;

use crate::{types::GqlDirectiveDefinition, Schema};

use super::input_value::__InputValue;

pub(crate) struct __Directive<'a> {
    pub schema: &'a Schema,
    pub detail: GqlDirectiveDefinition,
}

impl<'a> __Directive<'a> {
    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&String> {
        self.detail.description.as_ref()
    }

    async fn locations(&self) -> &Vec<DirectiveLocation> {
        &self.detail.locations
    }

    async fn args(&'a self) -> Vec<__InputValue<'a>> {
        let mut result = Vec::new();

        for arg in &self.detail.arguments {
            let value = __InputValue::new(self.schema, arg);
            result.push(value);
        }
        result
    }
}
