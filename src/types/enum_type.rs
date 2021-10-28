use graphql_parser::schema::{EnumType};

use super::GraphQLDirective;

#[derive(Debug, Clone)]
pub struct GraphQLEnum {
    name: String,
    description: Option<String>,
    values: Vec<String>,
    directives: Vec<GraphQLDirective>,
}

impl GraphQLEnum {
    pub fn parse<'a>(input: EnumType<'a, &'a str>) -> Self {
        let values: Vec<String> = input
            .values
            .into_iter()
            .map(|v| v.name.to_string())
            .collect();
        let directives = input
            .directives
            .into_iter()
            .map(|dir| GraphQLDirective::parse(dir))
            .collect();

        GraphQLEnum {
            name: input.name.to_string(),
            description: input.description,
            values,
            directives,
        }
    }
}
