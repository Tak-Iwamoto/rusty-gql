use graphql_parser::schema::Type;

#[derive(Debug, Clone)]
pub enum GqlMetaTypeName {
    NamedType(String),
    ListType(Box<GqlMetaTypeName>),
    NonNullType(Box<GqlMetaTypeName>),
}

impl GqlMetaTypeName {
    pub fn to_parser_type<'a>(&self) -> Type<'a, String> {
        match self {
            GqlMetaTypeName::NamedType(name) => Type::NamedType(name.clone()),
            GqlMetaTypeName::ListType(list) => Type::ListType(Box::new(list.to_parser_type())),
            GqlMetaTypeName::NonNullType(non_null) => {
                Type::NonNullType(Box::new(non_null.to_parser_type()))
            }
        }
    }

    pub fn is_non_null(&self) -> bool {
        matches!(self, &GqlMetaTypeName::NonNullType(_))
    }
}

impl<'a> From<Type<'a, String>> for GqlMetaTypeName {
    fn from(meta_type: Type<'a, String>) -> Self {
        match meta_type {
            Type::NamedType(named_type) => GqlMetaTypeName::NamedType(named_type),
            Type::ListType(list_type) => {
                GqlMetaTypeName::ListType(Box::new(Self::from(*list_type)))
            }
            Type::NonNullType(non_null) => {
                GqlMetaTypeName::NonNullType(Box::new(Self::from(*non_null)))
            }
        }
    }
}
