use super::{
    gql_enum::GqlEnum, gql_union::GqlUnion, input_object::GqlInputObject, interface::GqlInterface,
    object::GqlObject, scalar::GqlScalar,
};

#[derive(Debug, Clone)]
pub enum GqlMetaType {
    Scalar(GqlScalar),
    Object(GqlObject),
    Interface(GqlInterface),
    Union(GqlUnion),
    Enum(GqlEnum),
    Input(GqlInputObject),
    List(Box<GqlMetaType>),
}
