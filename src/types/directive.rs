use graphql_parser::schema::{Directive, Text};

use super::argument::GraphQLArgument;

#[derive(Debug)]
pub struct GraphQLDirective {
    pub name: String,
    // args: Vec<GraphQLArgument>,
}

impl GraphQLDirective {
    pub fn parse<'a, T: Text<'a>>(input: Directive<'a, T>) -> GraphQLDirective {
        for arg in input.arguments {

        }

        GraphQLDirective {
            name: format!("{:?}", input.name),
        }
    }
}
