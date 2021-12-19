use crate::{GqlField, Schema};

use super::{input_value::__InputValue, introspection_type::__Type};

// type __Field {
//   name: String!
//   description: String
//   args: [__InputValue!]!
//   type: __Type!
//   isDeprecated: Boolean!
//   deprecationReason: String
// }

pub(crate) struct __Field<'a> {
    schema: &'a Schema,
    detail: GqlField,
}

impl<'a> __Field<'a> {
    pub fn new(schema: &'a Schema, field: GqlField) -> Self {
        __Field {
            schema,
            detail: field,
        }
    }

    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&String> {
        self.detail.description.as_ref()
    }

    async fn args(&'a self) -> Vec<__InputValue<'a>> {
        let mut result = Vec::new();

        for arg in &self.detail.arguments {
            let value = __InputValue::new(self.schema, arg);
            result.push(value);
        }
        result
    }

    async fn ty(&'a self) -> __Type<'a> {
        __Type::from_value_type(self.schema, &self.detail.meta_type)
    }

    async fn is_deprecated(&self) -> bool {
        self.detail.is_deprecated()
    }
}
