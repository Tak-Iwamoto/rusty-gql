use graphql_parser::schema::Type;

#[derive(Debug, Clone)]
pub enum GqlValueType {
    NamedType(String),
    ListType(Box<GqlValueType>),
    NonNullType(Box<GqlValueType>),
}

impl GqlValueType {
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
}

impl<'a> From<Type<'a, String>> for GqlValueType {
    fn from(meta_type: Type<'a, String>) -> Self {
        match meta_type {
            Type::NamedType(named_type) => GqlValueType::NamedType(named_type),
            Type::ListType(list_type) => {
                GqlValueType::ListType(Box::new(Self::from(*list_type)))
            }
            Type::NonNullType(non_null) => {
                GqlValueType::NonNullType(Box::new(Self::from(*non_null)))
            }
        }
    }
}
