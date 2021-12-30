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

    pub fn to_rust_type_str(&self) -> String {
        match self {
            GqlValueType::NamedType(name) => gql_to_rust_type_str(name, true),
            GqlValueType::ListType(list_type) => gql_to_rust_type_str(list_type.name(), false),
            GqlValueType::NonNullType(non_null_type) => {
                gql_to_rust_type_str(non_null_type.name(), false)
            }
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

fn gql_to_rust_type_str(gql_type: &str, optional: bool) -> String {
    match gql_type {
        "Int" => format_rust_type("i64", optional),
        "Float" => format_rust_type("f64", optional),
        "String" => format_rust_type("String", optional),
        "Boolean" => format_rust_type("bool", optional),
        _ => format_rust_type(gql_type, optional),
    }
}

fn format_rust_type(type_name: &str, optional: bool) -> String {
    if optional {
        format!("Option<{}>", type_name)
    } else {
        type_name.to_string()
    }
}
