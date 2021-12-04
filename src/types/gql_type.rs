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
    InputObject(GqlInputObject),
    List(Box<GqlMetaType>),
}

impl ToString for GqlMetaType {
    fn to_string(&self) -> String {
        match self {
            GqlMetaType::Scalar(_) => "Scalar".to_string(),
            GqlMetaType::Object(_) => "Object".to_string(),
            GqlMetaType::Interface(_) => "Interface".to_string(),
            GqlMetaType::Union(_) => "Union".to_string(),
            GqlMetaType::Enum(_) => "Enum".to_string(),
            GqlMetaType::InputObject(_) => "InputObject".to_string(),
            GqlMetaType::List(_) => "List".to_string(),
        }
    }
}

impl GqlMetaType {
    pub fn is_input_type(&self) -> bool {
        matches!(self, &GqlMetaType::Scalar(_))
            || matches!(self, &GqlMetaType::InputObject(_))
            || matches!(self, &GqlMetaType::Enum(_))
    }
}
