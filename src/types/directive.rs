use std::collections::HashMap;

use graphql_parser::schema::{Directive, DirectiveDefinition};

use super::argument::GraphQLArgument;

#[derive(Debug)]
pub struct GraphQLDirective {
    pub name: String,
    pub description: Option<String>,
    pub args: HashMap<String, String>,
}

impl GraphQLDirective {
    pub fn parse<'a>(input: Directive<'a, &'a str>) -> GraphQLDirective {
        let mut args_map = HashMap::new();
        for arg in input.arguments {
            args_map.insert(arg.0.to_string(), arg.1.to_string());
        }
        GraphQLDirective {
            name: input.name.to_string(),
            description: None,
            args: args_map,
        }
    }
}

#[derive(Debug)]
pub struct GraphQLDirectiveDefinition {
    pub name: String,
    pub description: Option<String>,
    pub args: Vec<GraphQLArgument>,
}

impl GraphQLDirectiveDefinition {
    pub fn parse<'a>(input: DirectiveDefinition<'a, &'a str>) -> Self {
        let args = input
            .arguments
            .into_iter()
            .map(|arg| GraphQLArgument::parse(arg))
            .collect();

        GraphQLDirectiveDefinition {
            name: input.name.to_string(),
            description: input.description,
            args,
        }
    }
}
