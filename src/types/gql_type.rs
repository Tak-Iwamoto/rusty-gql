use super::{
    enum_type::GraphQLEnum, input::GraphQLInput, interface::GraphQLInterface,
    object::GraphQLObject, scalar::GraphQLScalar, union_type::GraphQLUnion,
};

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
