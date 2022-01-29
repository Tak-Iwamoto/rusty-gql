use graphql_parser::schema::Field;
use graphql_parser::Pos;

use super::argument::InputValueType;
use super::directive::GqlDirective;
use super::value_type::GqlValueType;

#[derive(Debug, Clone)]
pub struct FieldType {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub meta_type: GqlValueType,
    pub arguments: Vec<InputValueType>,
    pub directives: Vec<GqlDirective>,
}

impl FieldType {
    pub fn from_vec_field(fields: Vec<Field<'_, String>>) -> Vec<FieldType> {
        fields.into_iter().map(FieldType::from).collect()
    }

    pub fn is_deprecated(&self) -> bool {
        for dir in &self.directives {
            if dir.name == "deprecated" {
                return true;
            }
            continue;
        }
        false
    }
}

impl<'a> From<Field<'a, String>> for FieldType {
    fn from(field: Field<'a, String>) -> Self {
        let meta_type = GqlValueType::from(field.field_type);
        let directives = GqlDirective::from_vec_directive(field.directives);
        let arguments = InputValueType::from_vec_input_value(field.arguments);

        FieldType {
            name: field.name,
            description: field.description,
            position: field.position,
            meta_type,
            directives,
            arguments,
        }
    }
}
