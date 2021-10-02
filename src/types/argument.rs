use graphql_parser::schema::{InputValue, Text};

use super::{directive, gql_type::GraphQLGenericType, GraphQLDirective};

#[derive(Debug)]
pub struct GraphQLArgument {
    pub name: String,
    pub description: Option<String>,
    pub arg_type: GraphQLGenericType,
    // TODO: parse directive
    // pub directives: Vec<GraphQLDirective>,
}

impl GraphQLArgument {
    pub fn parse<'a>(input: InputValue<'a, &'a str>) -> GraphQLArgument {
        let name = input.name.into();
        let arg_type = GraphQLGenericType::parse(input.value_type);
        GraphQLArgument {
            name,
            description: input.description,
            arg_type,
        }
    }
}
