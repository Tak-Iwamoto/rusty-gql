use graphql_parser::schema::{
    EnumType, InputObjectType, InterfaceType, ObjectType, ScalarType, UnionType,
};

#[derive(Debug, Clone)]
pub enum GraphQLType<'a> {
    Null,
    NonNull(Box<GraphQLType<'a>>),
    Scalar(ScalarType<'a, String>),
    Object(ObjectType<'a, String>),
    Interface(InterfaceType<'a, String>),
    Union(UnionType<'a, String>),
    Enum(EnumType<'a, String>),
    Input(InputObjectType<'a, String>),
    List(Box<GraphQLType<'a>>),
}
