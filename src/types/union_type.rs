use graphql_parser::schema::UnionType;

use super::GraphQLDirective;

#[derive(Debug)]
pub struct GraphQLUnion {
    pub name: String,
    pub description: Option<String>,
    pub types: Vec<String>,
    // TODO:
    // directives: Vec<GraphQLDirective>,
}

impl GraphQLUnion {
    pub fn parse<'a>(input: UnionType<'a, &'a str>) -> Self {
        let types = input.types.into_iter().map(|t| t.to_string()).collect();
        GraphQLUnion {
            name: input.name.to_string(),
            description: input.description,
            types,
        }
    }
}
