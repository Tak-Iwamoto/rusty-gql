use std::collections::BTreeMap;

use graphql_parser::{query::Selection, schema::{Directive, Value}};

#[derive(Clone, Debug, PartialEq)]
pub enum GraphQLValue {
    Null,
    String(String),
    Boolean(bool),
    Int(i64),
    Float(f64),
    Enum(String),
    List(Vec<GraphQLValue>),
    Object(BTreeMap<String, GraphQLValue>),
}
