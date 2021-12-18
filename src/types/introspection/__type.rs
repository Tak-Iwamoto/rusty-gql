use crate::{types::GqlValueType, GqlTypeDefinition, Schema};

use super::__field::__Field;

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
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
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
        __Type { schema, detail }
    }

    async fn kind(&self) -> __TypeKind {
        match self.detail {
            TypeDetail::Named(def) => match def {
                GqlTypeDefinition::Scalar(_) => __TypeKind::Scalar,
                GqlTypeDefinition::Object(_) => __TypeKind::Object,
                GqlTypeDefinition::Interface(_) => __TypeKind::Interface,
                GqlTypeDefinition::Union(_) => __TypeKind::Union,
                GqlTypeDefinition::Enum(_) => __TypeKind::Enum,
                GqlTypeDefinition::InputObject(_) => __TypeKind::InputObject,
            },
            TypeDetail::NonNull(_) => __TypeKind::NonNull,
            TypeDetail::List(_) => __TypeKind::List,
        }
    }

    async fn name(&self) -> Option<&str> {
        match self.detail {
            TypeDetail::Named(def) => Some(def.name()),
            TypeDetail::NonNull(_) => None,
            TypeDetail::List(_) => None,
        }
    }

    async fn description(&self) -> Option<&String> {
        match self.detail {
            TypeDetail::Named(def) => def.description().as_ref(),
            TypeDetail::NonNull(_) => None,
            TypeDetail::List(_) => None,
        }
    }

    async fn fields(&self) -> Option<Vec<__Field<'a>>> {
        if let TypeDetail::Named(def) = self.detail {
            match def.fields() {
                Some(fields) => {
                    let result = fields
                        .into_iter()
                        .map(|field| __Field::new(self.schema, field.clone()))
                        .collect();
                    Some(result)
                }
                None => None,
            }
        } else {
            None
        }
    }

    async fn interfaces(&self) -> Option<Vec<__Type<'a>>> {
        if let TypeDetail::Named(def) = self.detail {
            if let GqlTypeDefinition::Object(obj) = def {
                let mut interfaces = Vec::new();

                for interface_name in &obj.implements_interfaces {
                    match self.schema.type_definitions.get(interface_name) {
                        Some(def) => {
                            let ty = __Type::from_type_definition(self.schema, def);
                            interfaces.push(ty);
                        }
                        None => continue,
                    }
                }
                Some(interfaces)
            } else {
                None
            }
        } else {
            None
        }
    }
}
