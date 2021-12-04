use graphql_parser::schema::Type;

#[derive(Debug, Clone)]
pub enum GqlMetaTypeName {
    NamedType(String),
    ListType(Box<GqlMetaTypeName>),
    NonNullType(Box<GqlMetaTypeName>),
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
