use std::collections::BTreeMap;

use graphql_parser::{schema::Directive, Pos};

use super::{argument::GqlArgument, value::GqlValue};

#[derive(Debug, Clone)]
pub struct GqlDirective {
    pub position: Pos,
    pub name: String,
    pub arguments: BTreeMap<String, GqlValue>,
}

impl GqlDirective {
    pub fn from_vec_directive<'a>(directives: Vec<Directive<'a, String>>) -> Vec<GqlDirective> {
        directives
            .into_iter()
            .map(|dir| GqlDirective::from(dir))
            .collect()
    }
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

pub fn convert_vec_directive<'a>(directives: Vec<Directive<'a, String>>) -> Vec<GqlDirective> {
    directives
        .into_iter()
        .map(|dir| GqlDirective::from(dir))
        .collect()
}

#[derive(Debug)]
pub struct GqlDirectiveDefinition {
    pub position: Pos,
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<GqlArgument>,
}
