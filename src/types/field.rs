use graphql_parser::schema::Field;
use graphql_parser::Pos;

use super::argument::GqlArgument;
use super::directive::GqlDirective;
use super::meta_type::GqlMetaType;

#[derive(Debug)]
pub struct GqlField {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub meta_type: GqlMetaType,
    pub arguments: Vec<GqlArgument>,
    pub directives: Vec<GqlDirective>,
}

impl<'a> From<Field<'a, String>> for GqlField {
    fn from(field: Field<'a, String>) -> Self {
        let meta_type = GqlMetaType::from(field.field_type);
        let directives = field
            .directives
            .into_iter()
            .map(|dir| GqlDirective::from(dir))
            .collect();

        let arguments = field
            .arguments
            .into_iter()
            .map(|arg| GqlArgument::from(arg))
            .collect();

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
