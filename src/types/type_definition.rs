use graphql_parser::schema::TypeDefinition as ParserTypeDefinition;

use crate::{FieldType, GqlDirective, Schema};

use super::{
    enum_type::EnumType, input_object::InputObjectType, interface::InterfaceType,
    object::ObjectType, scalar::ScalarType, union_type::UnionType,
};

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    Scalar(ScalarType),
    Object(ObjectType),
    Interface(InterfaceType),
    Union(UnionType),
    Enum(EnumType),
    InputObject(InputObjectType),
}

impl ToString for TypeDefinition {
    fn to_string(&self) -> String {
        match self {
            TypeDefinition::Scalar(_) => "Scalar".to_string(),
            TypeDefinition::Object(_) => "Object".to_string(),
            TypeDefinition::Interface(_) => "Interface".to_string(),
            TypeDefinition::Union(_) => "Union".to_string(),
            TypeDefinition::Enum(_) => "Enum".to_string(),
            TypeDefinition::InputObject(_) => "InputObject".to_string(),
        }
    }
}

impl TypeDefinition {
    pub fn from_schema_type_def<'a>(ty_def: &ParserTypeDefinition<'a, String>) -> Self {
        match ty_def {
            ParserTypeDefinition::Scalar(v) => TypeDefinition::Scalar(ScalarType::from(v.clone())),
            ParserTypeDefinition::Object(v) => TypeDefinition::Object(ObjectType::from(v.clone())),
            ParserTypeDefinition::Interface(v) => {
                TypeDefinition::Interface(InterfaceType::from(v.clone()))
            }
            ParserTypeDefinition::Union(v) => TypeDefinition::Union(UnionType::from(v.clone())),
            ParserTypeDefinition::Enum(v) => TypeDefinition::Enum(EnumType::from(v.clone())),
            ParserTypeDefinition::InputObject(v) => {
                TypeDefinition::InputObject(InputObjectType::from(v.clone()))
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            TypeDefinition::Scalar(scalar) => &scalar.name,
            TypeDefinition::Object(obj) => &obj.name,
            TypeDefinition::Interface(interface) => &interface.name,
            TypeDefinition::Union(uni) => &uni.name,
            TypeDefinition::Enum(enu) => &enu.name,
            TypeDefinition::InputObject(input_object) => &input_object.name,
        }
    }

    pub fn description(&self) -> &Option<String> {
        match self {
            TypeDefinition::Scalar(scalar) => &scalar.description,
            TypeDefinition::Object(obj) => &obj.description,
            TypeDefinition::Interface(interface) => &interface.description,
            TypeDefinition::Union(uni) => &uni.description,
            TypeDefinition::Enum(enu) => &enu.description,
            TypeDefinition::InputObject(input_object) => &input_object.description,
        }
    }

    pub fn fields(&self) -> Option<&Vec<FieldType>> {
        match self {
            TypeDefinition::Object(obj) => Some(&obj.fields),
            TypeDefinition::Interface(interface) => Some(&interface.fields),
            _ => None,
        }
    }

    pub fn get_field_by_name(&self, name: &str) -> Option<&FieldType> {
        self.fields()
            .and_then(|fields| fields.into_iter().find(|f| f.name == name))
    }

    pub fn is_composite_type(&self) -> bool {
        matches!(
            self,
            &TypeDefinition::Object(_) | &TypeDefinition::Interface(_) | &TypeDefinition::Union(_)
        )
    }

    pub fn is_input_type(&self) -> bool {
        matches!(
            self,
            &TypeDefinition::Scalar(_) | &TypeDefinition::InputObject(_) | &TypeDefinition::Enum(_)
        )
    }
    pub fn is_leaf_type(&self) -> bool {
        matches!(self, &TypeDefinition::Enum(_) | &TypeDefinition::Scalar(_))
    }

    pub fn directives(&self) -> &[GqlDirective] {
        match self {
            TypeDefinition::Scalar(ty) => &ty.directives,
            TypeDefinition::Object(ty) => &ty.directives,
            TypeDefinition::Interface(ty) => &ty.directives,
            TypeDefinition::Union(ty) => &ty.directives,
            TypeDefinition::Enum(ty) => &ty.directives,
            TypeDefinition::InputObject(ty) => &ty.directives,
        }
    }

    pub fn field_directives(&self, field_name: &str) -> Vec<GqlDirective> {
        let mut directives = vec![];

        if let TypeDefinition::Object(obj) = self {
            for field in &obj.fields {
                if field.name == field_name {
                    directives.extend(field.directives.clone());
                }
            }
        }
        if let TypeDefinition::Interface(interface) = self {
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

        if let TypeDefinition::Object(obj) = self {
            for impl_interface in &obj.implements_interfaces {
                if let Some(interface) = &schema.interfaces.get(impl_interface) {
                    directives.extend(interface.directives.clone());
                }
            }
        }
        directives
    }
}
