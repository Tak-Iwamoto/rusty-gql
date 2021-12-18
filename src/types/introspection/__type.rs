use crate::{types::GqlValueType, GqlTypeDefinition, Schema};

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

pub(crate) enum TypeDetail<'a> {
    Named(&'a GqlTypeDefinition),
    NonNull(&'a str),
    List(&'a str),
}

pub(crate) struct __Type<'a> {
    schema: &'a Schema,
    detail: TypeDetail<'a>,
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

impl<'a> __Type<'a> {
    pub fn from_type_definition(
        schema: &'a Schema,
        type_definition: &'a GqlTypeDefinition,
    ) -> Self {
        __Type {
            schema,
            detail: TypeDetail::Named(type_definition),
        }
    }

    pub fn from_value_type(schema: &'a Schema, value_type: &'a GqlValueType) -> Self {
        let detail = match value_type {
            GqlValueType::NamedType(named) => {
                let type_def = schema.type_definitions.get(named);
                match type_def {
                    Some(def) => TypeDetail::Named(def),
                    None => panic!("Unknown type: '{}'", named),
                }
            }
            GqlValueType::ListType(list) => TypeDetail::List(list.name()),
            GqlValueType::NonNullType(non_null) => TypeDetail::NonNull(non_null.name()),
        };
        __Type {
            schema,
            detail,
        }
    }
}
