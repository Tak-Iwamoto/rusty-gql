use std::collections::BTreeMap;

use graphql_parser::{query::Selection, schema::Directive};

use crate::types::{GraphQLDirective, GraphQLField};

#[derive(Clone, Debug, PartialEq)]
pub enum GraphQLValue {
    Null,
    String(String),
    Boolean(bool),
    Int(i32),
    Float(f32),
    Enum(String),
    List(Vec<GraphQLValue>),
    Object(BTreeMap<String, GraphQLValue>),
}

pub fn get_directive_values<'a>(directive: GraphQLDirective, node: Selection<'a, &'a str>) {
    let directveNode = match node {
        Selection::Field(field) => field
            .directives
            .into_iter()
            .find(|dir| dir.name == directive.name),
        Selection::FragmentSpread(frg_spread) => frg_spread
            .directives
            .into_iter()
            .find(|dir| dir.name == directive.name),
        Selection::InlineFragment(inline_frg) => inline_frg
            .directives
            .into_iter()
            .find(|dir| dir.name == directive.name),
    };
}

pub fn get_argument_values<'a>(def: GraphQLField, node: Directive<'a, &'a str>) {}
