use graphql_parser::schema::{InputValue, Text};

use super::gql_type::{GraphQLGenericType, GraphQLType};

#[derive(Debug)]
pub struct GraphQLArgument {
    pub name: String,
    pub description: Option<String>,
    pub arg_type: GraphQLGenericType,
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
