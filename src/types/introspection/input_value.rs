use rusty_gql_macro::Resolver;

use crate::{GqlArgument, Schema, SelectionSetResolver};

use super::introspection_type::__Type;

pub struct __InputValue<'a> {
    schema: &'a Schema,
    detail: GqlArgument,
}
pub fn build_input_value_introspection<'a>(
    schema: &'a Schema,
    value: &'a GqlArgument,
) -> __InputValue<'a> {
    __InputValue {
        schema,
        detail: value.clone(),
    }
}

#[Resolver(internal)]
impl<'a> __InputValue<'a> {
    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&str> {
        self.detail.description.as_deref()
    }

    async fn ty(&'a self) -> __Type<'a> {
        __Type::from_value_type(self.schema, &self.detail.meta_type)
    }

    async fn default_value(&self) -> Option<String> {
        match &self.detail.default_value {
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }
}
