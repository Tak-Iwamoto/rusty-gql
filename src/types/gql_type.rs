use super::{enum_type::GraphQLEnum, input::GraphQLInput, interface::GraphQLInterface, object::GraphQLObject, scalar::GraphQLScalar, union_type::GraphQLUnion};

pub enum GraphQLType {
    GraphQLScalar(GraphQLScalar),
    GraphQLObject(GraphQLObject),
    GraphQLInterface(GraphQLInterface),
    GraphQLUnion(GraphQLUnion),
    GraphQLEnum(GraphQLEnum),
    GraphQLInput(GraphQLInput),
    GraphQLList(Vec<GraphQLType>),
}
