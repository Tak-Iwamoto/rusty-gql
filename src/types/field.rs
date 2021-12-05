use graphql_parser::schema::Field;
use graphql_parser::Pos;

use super::argument::GqlArgument;
use super::directive::GqlDirective;
use super::value_type::GqlValueType;

#[derive(Debug, Clone)]
pub struct GqlField {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub meta_type: GqlValueType,
    pub arguments: Vec<GqlArgument>,
    pub directives: Vec<GqlDirective>,
}

impl GqlField {
    pub fn from_vec_field<'a>(fields: Vec<Field<'a, String>>) -> Vec<GqlField> {
        fields
            .into_iter()
            .map(|field| GqlField::from(field))
            .collect()
    }
}

impl<'a> From<Field<'a, String>> for GqlField {
    fn from(field: Field<'a, String>) -> Self {
        let meta_type = GqlValueType::from(field.field_type);
        let directives = GqlDirective::from_vec_directive(field.directives);
        let arguments = GqlArgument::from_vec_input_value(field.arguments);

        GqlField {
            name: field.name,
            description: field.description,
            position: field.position,
            meta_type,
            directives,
            arguments,
        }
    }
}
