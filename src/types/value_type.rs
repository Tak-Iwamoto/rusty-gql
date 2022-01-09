use graphql_parser::schema::Type;

use crate::GqlValue;

#[derive(Debug, Clone)]
pub enum GqlValueType {
    NamedType(String),
    ListType(Box<GqlValueType>),
    NonNullType(Box<GqlValueType>),
}

impl GqlValueType {
    pub fn name(&self) -> &str {
        match self {
            GqlValueType::NamedType(name) => name,
            GqlValueType::ListType(list_type) => list_type.name(),
            GqlValueType::NonNullType(non_null_type) => non_null_type.name(),
        }
    }

    pub fn to_parser_type<'a>(&self) -> Type<'a, String> {
        match self {
            GqlValueType::NamedType(name) => Type::NamedType(name.clone()),
            GqlValueType::ListType(list) => Type::ListType(Box::new(list.to_parser_type())),
            GqlValueType::NonNullType(non_null) => {
                Type::NonNullType(Box::new(non_null.to_parser_type()))
            }
        }
    }

    pub fn is_non_null(&self) -> bool {
        matches!(self, &GqlValueType::NonNullType(_))
    }

    pub fn is_sub_type(&self, sub: &GqlValueType, default_value: &Option<GqlValue>) -> bool {
        match (self, sub) {
            (GqlValueType::NonNullType(base_type), GqlValueType::NonNullType(sub_type)) => {
                base_type.is_sub_type(&*sub_type, default_value)
            }
            (GqlValueType::NamedType(base_type_name), GqlValueType::NonNullType(sub_type)) => {
                base_type_name.eq(&sub_type.name())
            }
            (GqlValueType::NonNullType(base_type), GqlValueType::NamedType(sub_type)) => {
                if let Some(default) = default_value {
                    base_type.name().eq(sub_type)
                } else {
                    false
                }
            }
            (GqlValueType::NamedType(base_type_name), GqlValueType::NamedType(sub_type_name)) => {
                base_type_name.eq(sub_type_name)
            }
            (GqlValueType::ListType(base_type), GqlValueType::ListType(sub_type)) => {
                base_type.is_sub_type(&*sub_type, default_value)
            }
            _ => false,
        }
    }
}

impl<'a> From<Type<'a, String>> for GqlValueType {
    fn from(meta_type: Type<'a, String>) -> Self {
        match meta_type {
            Type::NamedType(named_type) => GqlValueType::NamedType(named_type),
            Type::ListType(list_type) => GqlValueType::ListType(Box::new(Self::from(*list_type))),
            Type::NonNullType(non_null) => {
                GqlValueType::NonNullType(Box::new(Self::from(*non_null)))
            }
        }
    }
}
