use graphql_parser::{schema::InputObjectType as ParserInputObjectType, Pos};

use super::{argument::InputValueType, directive::GqlDirective};

#[derive(Debug, Clone)]
pub struct InputObjectType {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
    pub fields: Vec<InputValueType>,
}

impl<'a> From<ParserInputObjectType<'a, String>> for InputObjectType {
    fn from(input_object: ParserInputObjectType<'a, String>) -> Self {
        let directives = input_object
            .directives
            .into_iter()
            .map(GqlDirective::from)
            .collect();

        let fields = input_object
            .fields
            .into_iter()
            .map(InputValueType::from)
            .collect();

        InputObjectType {
            name: input_object.name,
            description: input_object.description,
            position: input_object.position,
            directives,
            fields,
        }
    }
}
