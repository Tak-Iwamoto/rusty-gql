use std::collections::BTreeMap;

use graphql_parser::{schema::Directive, Pos};

use super::{argument::GqlArgument, value::GqlValue};

#[derive(Debug)]
pub struct GqlDirective {
    pub position: Pos,
    pub name: String,
    pub arguments: BTreeMap<String, GqlValue>,
}

impl<'a> From<Directive<'a, String>> for GqlDirective {
    fn from(directive: Directive<'a, String>) -> Self {
        let mut arguments = BTreeMap::new();
        for (key, value) in directive.arguments {
            let gql_value = GqlValue::from(value);
            arguments.insert(key, gql_value);
        }
        GqlDirective {
            position: directive.position,
            name: directive.name,
            arguments,
        }
    }
}

#[derive(Debug)]
pub struct GqlDirectiveDefinition {
    pub position: Pos,
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<GqlArgument>,
}
