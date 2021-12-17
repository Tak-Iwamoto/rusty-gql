use crate::Schema;

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
}
