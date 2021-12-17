use crate::Schema;

// type __EnumValue {
//   name: String!
//   description: String
//   isDeprecated: Boolean!
//   deprecationReason: String
// }

pub(crate) struct __EnumValue {
    schema: &Schema,
}
