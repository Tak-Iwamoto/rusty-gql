use graphql_parser::schema::{Field, TypeDefinition};

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
    pub fn type_name_from_def<'a>(type_definition: &TypeDefinition<'a, String>) -> String {
        match type_definition {
            TypeDefinition::Scalar(scalar) => scalar.name.clone(),
            TypeDefinition::Object(obj) => obj.name.clone(),
            TypeDefinition::Interface(interface) => interface.name.clone(),
            TypeDefinition::Union(uni) => uni.name.clone(),
            TypeDefinition::Enum(enu) => enu.name.clone(),
            TypeDefinition::InputObject(input_obj) => input_obj.name.clone(),
        }
    }

    pub fn fields_from_def<'a>(
        type_definition: &TypeDefinition<'a, String>,
    ) -> Option<Vec<Field<'a, String>>> {
        match type_definition {
            TypeDefinition::Object(obj) => Some(obj.fields.clone()),
            TypeDefinition::Interface(interface) => Some(interface.fields.clone()),
            _ => None,
        }
    }

    pub fn get_field_by_name<'a>(
        type_definition: &TypeDefinition<'a, String>,
        name: &str,
    ) -> Option<Field<'a, String>> {
        GqlTypeDefinition::fields_from_def(type_definition)
            .and_then(|fields| fields.iter().find(|f| f.name == name).map(Clone::clone))
    }

    pub fn name(&self) -> &str {
        match self {
            GqlTypeDefinition::Scalar(scalar) => &scalar.name,
            GqlTypeDefinition::Object(obj) => &obj.name,
            GqlTypeDefinition::Interface(interface) => &interface.name,
            GqlTypeDefinition::Union(uni) => &uni.name,
            GqlTypeDefinition::Enum(enu) => &enu.name,
            GqlTypeDefinition::InputObject(input_object) => &input_object.name,
            GqlTypeDefinition::List(list) => &list.name(),
        }
    }

    pub fn is_input_type(&self) -> bool {
        matches!(
            self,
            &GqlTypeDefinition::Scalar(_)
                | &GqlTypeDefinition::InputObject(_)
                | &GqlTypeDefinition::Enum(_)
        )
    }

    pub fn is_composite_type(&self) -> bool {
        matches!(
            self,
            &GqlTypeDefinition::Object(_)
                | &GqlTypeDefinition::Interface(_)
                | &GqlTypeDefinition::Union(_)
        )
    }

    pub fn is_leaf_type(&self) -> bool {
        matches!(
            self,
            &GqlTypeDefinition::Enum(_) | &GqlTypeDefinition::Scalar(_)
        )
    }

    pub fn fields(&self) -> Option<&Vec<GqlField>> {
        match self {
            GqlTypeDefinition::Object(obj) => Some(&obj.fields),
            GqlTypeDefinition::Interface(interface) => Some(&interface.fields),
            _ => None,
        }
    }
}
