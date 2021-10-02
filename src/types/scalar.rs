use graphql_parser::schema::ScalarType;

#[derive(Debug)]
pub struct GraphQLScalar {
    name: String,
    description: Option<String>,
    // TODO: parse directive
    // pub directives: Vec<GraphQLDirective>,
}

impl GraphQLScalar {
    pub fn parse<'a>(input: ScalarType<'a, &'a str>) -> Self {
        GraphQLScalar {
            name: input.name.to_string(),
            description: input.description,
        }
    }
}
