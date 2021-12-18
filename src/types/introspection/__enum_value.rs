use crate::{types::GqlEnumValue, Schema};

// type __EnumValue {
//   name: String!
//   description: String
//   isDeprecated: Boolean!
//   deprecationReason: String
// }

pub(crate) struct __EnumValue<'a> {
    schema: &'a Schema,
    detail: GqlEnumValue,
}

impl<'a> __EnumValue<'a> {
    pub fn new(schema: &'a Schema, value: &'a GqlEnumValue) -> Self {
        Self {
            schema,
            detail: value.clone(),
        }
    }
}
