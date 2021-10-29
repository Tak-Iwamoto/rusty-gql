use graphql_parser::schema::{
    EnumType, InputObjectType, InterfaceType, ObjectType, ScalarType, Type, UnionType,
};

#[derive(Debug, Clone)]
pub enum WrapType {
    NamedType(String),
    ListType(String),
    NonNullType(String),
}

impl WrapType {
    pub fn parse<'a>(input_type: Type<'a, &'a str>) -> WrapType {
        match input_type {
            Type::NamedType(named_type) => WrapType::NamedType(named_type.into()),
            Type::ListType(list_type) => WrapType::ListType(list_type.to_string()),
            Type::NonNullType(non_null) => WrapType::NonNullType(non_null.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GraphQLType<'a> {
    Null,
    NonNull(Box<GraphQLType<'a>>),
    Scalar(ScalarType<'a, &'a str>),
    Object(ObjectType<'a, &'a str>),
    Interface(InterfaceType<'a, &'a str>),
    Union(UnionType<'a, &'a str>),
    Enum(EnumType<'a, &'a str>),
    Input(InputObjectType<'a, &'a str>),
    List(Box<GraphQLType<'a>>),
}
