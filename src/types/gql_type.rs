use graphql_parser::schema::Type;

use super::{
    enum_type::GraphQLEnum, input::GraphQLInput, interface::GraphQLInterface,
    object::GraphQLObject, scalar::GraphQLScalar, union_type::GraphQLUnion,
};

#[derive(Debug)]
pub enum GraphQLGenericType {
    NamedType(String),
    ListType(String),
    NonNullType(String),
}

impl GraphQLGenericType {
    pub fn parse<'a>(input_type: Type<'a, &'a str>) -> GraphQLGenericType {
        match input_type {
            Type::NamedType(named_type) => GraphQLGenericType::NamedType(named_type.into()),
            Type::ListType(list_type) => GraphQLGenericType::ListType(list_type.to_string()),
            Type::NonNullType(non_null) => GraphQLGenericType::NonNullType(non_null.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum GraphQLType {
    Null,
    GraphQLScalar(GraphQLScalar),
    GraphQLObject(GraphQLObject),
    GraphQLInterface(GraphQLInterface),
    GraphQLUnion(GraphQLUnion),
    GraphQLEnum(GraphQLEnum),
    GraphQLInput(GraphQLInput),
    GraphQLList(Vec<GraphQLType>),
}
