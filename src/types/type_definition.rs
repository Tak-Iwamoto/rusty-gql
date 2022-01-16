use graphql_parser::schema::TypeDefinition;

use crate::{GqlDirective, GqlField, Schema};

use super::{
    enum_type::GqlEnum, input_object::GqlInputObject, interface::GqlInterface, object::GqlObject,
    scalar::GqlScalar, union_type::GqlUnion,
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

    pub fn fields(&self) -> Option<&Vec<GqlField>> {
        match self {
            GqlTypeDefinition::Object(obj) => Some(&obj.fields),
            GqlTypeDefinition::Interface(interface) => Some(&interface.fields),
            _ => None,
        }
    }

    pub fn get_field_by_name(&self, name: &str) -> Option<&GqlField> {
        self.fields()
            .and_then(|fields| fields.into_iter().find(|f| f.name == name))
    }

    pub fn is_composite_type(&self) -> bool {
        matches!(
            self,
            &GqlTypeDefinition::Object(_)
                | &GqlTypeDefinition::Interface(_)
                | &GqlTypeDefinition::Union(_)
        )
    }

    pub fn is_input_type(&self) -> bool {
        matches!(
            self,
            &GqlTypeDefinition::Scalar(_)
                | &GqlTypeDefinition::InputObject(_)
                | &GqlTypeDefinition::Enum(_)
        )
    }
    pub fn is_leaf_type(&self) -> bool {
        matches!(
            self,
            &GqlTypeDefinition::Enum(_) | &GqlTypeDefinition::Scalar(_)
        )
    }

    pub fn directives(&self) -> &[GqlDirective] {
        match self {
            GqlTypeDefinition::Scalar(ty) => &ty.directives,
            GqlTypeDefinition::Object(ty) => &ty.directives,
            GqlTypeDefinition::Interface(ty) => &ty.directives,
            GqlTypeDefinition::Union(ty) => &ty.directives,
            GqlTypeDefinition::Enum(ty) => &ty.directives,
            GqlTypeDefinition::InputObject(ty) => &ty.directives,
        }
    }

    pub fn field_directives(&self, field_name: &str) -> Vec<GqlDirective> {
        let mut directives = vec![];

        if let GqlTypeDefinition::Object(obj) = self {
            for field in &obj.fields {
                if field.name == field_name {
                    directives.extend(field.directives.clone());
                }
            }
        }
        if let GqlTypeDefinition::Interface(interface) = self {
            for field in &interface.fields {
                if field.name == field_name {
                    directives.extend(field.directives.clone());
                }
            }
        }
        directives
    }

    pub fn impl_interface_directives(&self, schema: &Schema) -> Vec<GqlDirective> {
        let mut directives = vec![];

        if let GqlTypeDefinition::Object(obj) = self {
            for impl_interface in &obj.implements_interfaces {
                if let Some(interface) = &schema.interfaces.get(impl_interface) {
                    directives.extend(interface.directives.clone());
                }
            }
        }
        directives
    }
}
