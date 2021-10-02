use graphql_parser::schema::{Directive, Text};

use super::argument::GraphQLArgument;

#[derive(Debug)]
pub struct GraphQLDirective {
    pub name: String,
    // TODO: directiveのargsを保存する
    // args: Vec<GraphQLArgument>,
}

impl GraphQLDirective {
    pub fn parse<'a>(input: Directive<'a, &'a str>) -> GraphQLDirective {
        GraphQLDirective {
            name: input.name.to_string(),
        }
    }
}
