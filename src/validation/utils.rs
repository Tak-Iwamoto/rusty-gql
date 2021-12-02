use std::collections::HashSet;

use graphql_parser::schema::{Directive, Value};

use super::visitor::ValidationContext;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope<'a> {
    Operation(Option<&'a str>),
    Fragment(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DirectiveLocation {
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
}

fn check_arg_uniqueness(ctx: &mut ValidationContext<'_>, args: &Vec<(String, Value<'_, String>)>) {
    for (arg_name, arg_value) in args {

    }
}
