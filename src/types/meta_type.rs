use graphql_parser::schema::Type;

#[derive(Debug)]
pub enum GqlMetaType {
    NamedType(String),
    ListType(Box<GqlMetaType>),
    NonNullType(Box<GqlMetaType>),
}

impl<'a> From<Type<'a, String>> for GqlMetaType {
    fn from(meta_type: Type<'a, String>) -> Self {
        match meta_type {
            Type::NamedType(named_type) => GqlMetaType::NamedType(named_type),
            Type::ListType(list_type) => GqlMetaType::ListType(Box::new(Self::from(*list_type))),
            Type::NonNullType(non_null) => {
                GqlMetaType::NonNullType(Box::new(Self::from(*non_null)))
            }
        }
    }
}
