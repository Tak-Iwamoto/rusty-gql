use graphql_parser::schema::ScalarType;

use super::GraphQLDirective;

#[derive(Debug)]
pub struct GraphQLScalar {
    pub name: String,
    pub description: Option<String>,
    pub directives: Vec<GraphQLDirective>,
}

impl GraphQLScalar {
    pub fn parse<'a>(input: ScalarType<'a, &'a str>) -> Self {
        let directives: Vec<GraphQLDirective> = input
            .directives
            .into_iter()
            .map(|dir| GraphQLDirective::parse(dir))
            .collect();

        GraphQLScalar {
            name: input.name.to_string(),
            description: input.description,
            directives,
        }
    }
}
