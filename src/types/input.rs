use graphql_parser::schema::InputObjectType;

use super::{argument::GraphQLArgument, GraphQLDirective};

#[derive(Debug)]
pub struct GraphQLInput {
    name: String,
    description: Option<String>,
    args: Vec<GraphQLArgument>,
    directives: Vec<GraphQLDirective>,
}

impl GraphQLInput {
    pub fn parse<'a>(input: InputObjectType<'a, &'a str>) -> Self {
        let args = input
            .fields
            .into_iter()
            .map(|f| GraphQLArgument::parse(f))
            .collect();
        let directives = input
            .directives
            .into_iter()
            .map(|dir| GraphQLDirective::parse(dir))
            .collect();

        GraphQLInput {
            name: input.name.to_string(),
            description: input.description,
            args,
            directives,
        }
    }
}
