use graphql_parser::schema::InputObjectType;

use super::argument::GraphQLArgument;

#[derive(Debug)]
pub struct GraphQLInput {
    name: String,
    args: Vec<GraphQLArgument>,
    description: Option<String>,
}

impl GraphQLInput {
    pub fn parse<'a>(input: InputObjectType<'a, &'a str>) -> Self {
        let args = input
            .fields
            .into_iter()
            .map(|f| GraphQLArgument::parse(f))
            .collect();
        GraphQLInput {
            name: input.name.to_string(),
            description: input.description,
            args,
        }
    }
}
