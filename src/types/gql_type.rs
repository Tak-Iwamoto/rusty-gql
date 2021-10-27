use graphql_parser::schema::Type;

use super::{
    enum_type::GraphQLEnum, input::GraphQLInput, interface::GraphQLInterface,
    object_type::GraphQLObjectType, scalar::GraphQLScalar, union_type::GraphQLUnion,
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
pub enum GraphQLType {
    Null,
    NonNull(Box<GraphQLType>),
    Scalar(GraphQLScalar),
    Object(GraphQLObjectType),
    Interface(GraphQLInterface),
    Union(GraphQLUnion),
    Enum(GraphQLEnum),
    Input(GraphQLInput),
    List(Box<GraphQLType>),
}
