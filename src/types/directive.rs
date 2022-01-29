use std::collections::BTreeMap;

use graphql_parser::{
    schema::{Directive, DirectiveLocation},
    Pos,
};

use crate::GqlValueType;

use super::{argument::InputValueType, value::GqlValue};

#[derive(Debug, Clone)]
pub struct GqlDirective {
    pub position: Pos,
    pub name: String,
    pub arguments: BTreeMap<String, GqlValue>,
}

impl GqlDirective {
    pub fn from_vec_directive(directives: Vec<Directive<'_, String>>) -> Vec<GqlDirective> {
        directives.into_iter().map(GqlDirective::from).collect()
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

#[derive(Debug)]
pub struct DirectiveDefinition {
    pub position: Pos,
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<InputValueType>,
    pub locations: Vec<DirectiveLocation>,
}

impl DirectiveDefinition {
    pub fn skip_directive() -> Self {
        DirectiveDefinition {
            position: Pos::default(),
            name: "skip".to_string(),
            description: None,
            arguments: vec![InputValueType {
                name: "if".to_string(),
                description: None,
                position: Pos::default(),
                meta_type: GqlValueType::NonNullType(Box::new(GqlValueType::NamedType(
                    "Boolean".to_string(),
                ))),
                default_value: None,
                directives: Default::default(),
            }],
            locations: vec![
                DirectiveLocation::Field,
                DirectiveLocation::FragmentSpread,
                DirectiveLocation::InlineFragment,
            ],
        }
    }

    pub fn include_directive() -> Self {
        DirectiveDefinition {
            position: Pos::default(),
            name: "include".to_string(),
            description: None,
            arguments: vec![InputValueType {
                name: "if".to_string(),
                description: None,
                position: Pos::default(),
                meta_type: GqlValueType::NonNullType(Box::new(GqlValueType::NamedType(
                    "Boolean".to_string(),
                ))),
                default_value: None,
                directives: Default::default(),
            }],
            locations: vec![
                DirectiveLocation::Field,
                DirectiveLocation::FragmentSpread,
                DirectiveLocation::InlineFragment,
            ],
        }
    }

    pub fn deprecated_directive() -> Self {
        DirectiveDefinition {
            position: Pos::default(),
            name: "deprecated".to_string(),
            description: None,
            arguments: vec![InputValueType {
                name: "reason".to_string(),
                description: None,
                position: Pos::default(),
                meta_type: GqlValueType::NamedType("String".to_string()),
                default_value: None,
                directives: Default::default(),
            }],
            locations: vec![
                DirectiveLocation::FieldDefinition,
                DirectiveLocation::EnumValue,
            ],
        }
    }
}
