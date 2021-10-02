use graphql_parser::schema::{InputValue, Text};

use super::gql_type::GraphQLType;

#[derive(Debug)]
pub struct GraphQLArgument {
    pub name: String,
    pub description: Option<String>,
    pub arg_type: GraphQLType,
}

impl GraphQLArgument {
    pub fn parse<'a>(input: InputValue<'a, &'a str>) -> GraphQLArgument {
        let name = input.name.into();
        GraphQLArgument {
            name,
            description: input.description,
            // TODO: value_typeから変換する
            arg_type: GraphQLType::Null,
        }
    }
}
