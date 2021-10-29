use graphql_parser::schema::{
    EnumType, InputObjectType, InterfaceType, ObjectType, ScalarType, UnionType,
};

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
