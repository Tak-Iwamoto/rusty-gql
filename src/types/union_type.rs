use graphql_parser::schema::UnionType;

use super::GraphQLDirective;

#[derive(Debug, Clone)]
pub struct GraphQLUnion {
    pub name: String,
    pub description: Option<String>,
    pub types: Vec<String>,
    pub directives: Vec<GraphQLDirective>,
}

impl GraphQLUnion {
    pub fn parse<'a>(input: UnionType<'a, &'a str>) -> Self {
        let types = input.types.into_iter().map(|t| t.to_string()).collect();
        let directives: Vec<GraphQLDirective> = input
            .directives
            .into_iter()
            .map(|dir| GraphQLDirective::parse(dir))
            .collect();
        GraphQLUnion {
            name: input.name.to_string(),
            description: input.description,
            types,
            directives,
        }
    }
}
