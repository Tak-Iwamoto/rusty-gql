use crate::GqlField;

use super::{
    gql_enum::GqlEnum, gql_union::GqlUnion, input_object::GqlInputObject, interface::GqlInterface,
    object::GqlObject, scalar::GqlScalar,
};

#[derive(Debug, Clone)]
pub enum GqlTypeDefinition {
    Scalar(GqlScalar),
    Object(GqlObject),
    Interface(GqlInterface),
    Union(GqlUnion),
    Enum(GqlEnum),
    InputObject(GqlInputObject),
    List(Box<GqlTypeDefinition>),
}

impl ToString for GqlTypeDefinition {
    fn to_string(&self) -> String {
        match self {
            GqlTypeDefinition::Scalar(_) => "Scalar".to_string(),
            GqlTypeDefinition::Object(_) => "Object".to_string(),
            GqlTypeDefinition::Interface(_) => "Interface".to_string(),
            GqlTypeDefinition::Union(_) => "Union".to_string(),
            GqlTypeDefinition::Enum(_) => "Enum".to_string(),
            GqlTypeDefinition::InputObject(_) => "InputObject".to_string(),
            GqlTypeDefinition::List(_) => "List".to_string(),
        }
    }
}

impl GqlTypeDefinition {
    pub fn is_input_type(&self) -> bool {
        matches!(
            self,
            &GqlTypeDefinition::Scalar(_) | &GqlTypeDefinition::InputObject(_) | &GqlTypeDefinition::Enum(_)
        )
    }

    pub fn is_composite_type(&self) -> bool {
        matches!(
            self,
            &GqlTypeDefinition::Object(_) | &GqlTypeDefinition::Interface(_) | &GqlTypeDefinition::Union(_)
        )
    }

    pub fn is_leaf_type(&self) -> bool {
        matches!(self, &GqlTypeDefinition::Enum(_) | &GqlTypeDefinition::Scalar(_))
    }

    pub fn fields(&self) -> Option<&Vec<GqlField>> {
        match self {
            GqlTypeDefinition::Object(obj) => Some(&obj.fields),
            GqlTypeDefinition::Interface(interface) => Some(&interface.fields),
            _ => None,
        }
    }

    pub fn get_field_by_name(&self, name: &str) -> Option<&GqlField> {
        self.fields()
            .and_then(|fields| fields.iter().find(|f| f.name == name))
    }
}
