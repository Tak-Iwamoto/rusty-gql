use graphql_parser::schema::Type;

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

    pub fn to_rust_type(&self) -> String {
        match self {
            GqlValueType::NamedType(name) => gql_to_rust_type(name),
            GqlValueType::ListType(list_type) => gql_to_rust_type(list_type.name()),
            GqlValueType::NonNullType(non_null_type) => gql_to_rust_type(non_null_type.name()),
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

pub fn gql_to_rust_type(gql_type: &str) -> String {
    match gql_type {
        "Int" => "i64".to_string(),
        "Float" => "f64".to_string(),
        "String" => "&str".to_string(),
        "Boolean" => "bool".to_string(),
        _ => gql_type.to_string(),
    }
}
