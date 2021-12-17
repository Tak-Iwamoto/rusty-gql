// type __Type {
//   kind: __TypeKind!
//   name: String
//   description: String
//   # must be non-null for OBJECT and INTERFACE, otherwise null.
//   fields(includeDeprecated: Boolean = false): [__Field!]
//   # must be non-null for OBJECT and INTERFACE, otherwise null.
//   interfaces: [__Type!]
//   # must be non-null for INTERFACE and UNION, otherwise null.
//   possibleTypes: [__Type!]
//   # must be non-null for ENUM, otherwise null.
//   enumValues(includeDeprecated: Boolean = false): [__EnumValue!]
//   # must be non-null for INPUT_OBJECT, otherwise null.
//   inputFields: [__InputValue!]
//   # must be non-null for NON_NULL and LIST, otherwise null.
//   ofType: __Type
//   # may be non-null for custom SCALAR, otherwise null.
//   specifiedByURL: String
// }

pub(crate) struct __Type {
    schema: &Schema,
}

#[allow(non_camel_case_types)]
pub(crate) enum __TypeKind {
    SCALAR,
    OBJECT,
    INTERFACE,
    UNION,
    ENUM,
    INPUT_OBJECT,
    LIST,
    NON_NULL,
}
