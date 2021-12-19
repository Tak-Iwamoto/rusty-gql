use crate::{GqlArgument, Schema};

use super::introspection_type::__Type;

pub(crate) struct __InputValue<'a> {
    schema: &'a Schema,
    detail: GqlArgument,
}

impl<'a> __InputValue<'a> {
    pub fn new(schema: &'a Schema, value: &'a GqlArgument) -> Self {
        Self {
            schema,
            detail: value.clone(),
        }
    }

    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&String> {
        self.detail.description.as_ref()
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
