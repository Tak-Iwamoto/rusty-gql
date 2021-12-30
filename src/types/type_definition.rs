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
        }
    }
}

impl GqlTypeDefinition {
    pub fn from_schema_type_def<'a>(ty_def: &TypeDefinition<'a, String>) -> Self {
        match ty_def {
            TypeDefinition::Scalar(v) => GqlTypeDefinition::Scalar(GqlScalar::from(v.clone())),
            TypeDefinition::Object(v) => GqlTypeDefinition::Object(GqlObject::from(v.clone())),
            TypeDefinition::Interface(v) => {
                GqlTypeDefinition::Interface(GqlInterface::from(v.clone()))
            }
            TypeDefinition::Union(v) => GqlTypeDefinition::Union(GqlUnion::from(v.clone())),
            TypeDefinition::Enum(v) => GqlTypeDefinition::Enum(GqlEnum::from(v.clone())),
            TypeDefinition::InputObject(v) => {
                GqlTypeDefinition::InputObject(GqlInputObject::from(v.clone()))
            }
        }
    }
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
        }
    }

    pub fn description(&self) -> &Option<String> {
        match self {
            GqlTypeDefinition::Scalar(scalar) => &scalar.description,
            GqlTypeDefinition::Object(obj) => &obj.description,
            GqlTypeDefinition::Interface(interface) => &interface.description,
            GqlTypeDefinition::Union(uni) => &uni.description,
            GqlTypeDefinition::Enum(enu) => &enu.description,
            GqlTypeDefinition::InputObject(input_object) => &input_object.description,
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
