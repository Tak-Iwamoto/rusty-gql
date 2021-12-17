use crate::Schema;

// type __InputValue {
//   name: String!
//   description: String
//   type: __Type!
//   defaultValue: String
// }

pub(crate) struct __InputValue<'a> {
    schema: &'a Schema,
}
