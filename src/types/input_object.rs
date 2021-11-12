use graphql_parser::{schema::InputObjectType, Pos};

use super::{argument::GqlArgument, directive::GqlDirective};

pub struct GqlInputObject {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
    pub fields: Vec<GqlArgument>,
}

impl<'a> From<InputObjectType<'a, String>> for GqlInputObject {
    fn from(input_object: InputObjectType<'a, String>) -> Self {
        let directives = input_object
            .directives
            .into_iter()
            .map(|dir| GqlDirective::from(dir))
            .collect();

        let fields = input_object
            .fields
            .into_iter()
            .map(|field| GqlArgument::from(field))
            .collect();

        GqlInputObject {
            name: input_object.name,
            description: input_object.description,
            position: input_object.position,
            directives,
            fields,
        }
    }
}
