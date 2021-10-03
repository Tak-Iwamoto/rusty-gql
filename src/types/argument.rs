use graphql_parser::schema::InputValue;

use super::{gql_type::GraphQLGenericType, GraphQLDirective};

#[derive(Debug)]
pub struct GraphQLArgument {
    pub name: String,
    pub description: Option<String>,
    pub arg_type: GraphQLGenericType,
    pub directives: Vec<GraphQLDirective>,
}

impl GraphQLArgument {
    pub fn parse<'a>(input: InputValue<'a, &'a str>) -> GraphQLArgument {
        let name = input.name.into();
        let arg_type = GraphQLGenericType::parse(input.value_type);
        let directives: Vec<GraphQLDirective> = input
            .directives
            .into_iter()
            .map(|dir| GraphQLDirective::parse(dir))
            .collect();
        GraphQLArgument {
            name,
            description: input.description,
            arg_type,
            directives,
        }
    }
}
