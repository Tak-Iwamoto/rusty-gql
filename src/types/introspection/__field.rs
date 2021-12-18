use crate::{GqlField, Schema};

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
}
