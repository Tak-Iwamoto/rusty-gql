use std::ops::RangeBounds;

use super::directive::GqlDirective;
use graphql_parser::{
    schema::{EnumType, EnumValue},
    Pos,
};

#[derive(Debug, Clone)]
pub struct GqlEnum {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
    pub values: Vec<GqlEnumValue>,
}

impl<'a> From<EnumType<'a, String>> for GqlEnum {
    fn from(gql_enum: EnumType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(gql_enum.directives);

        let values = gql_enum
            .values
            .into_iter()
            .map(|value| GqlEnumValue::from(value))
            .collect();

        GqlEnum {
            name: gql_enum.name,
            description: gql_enum.description,
            position: gql_enum.position,
            directives,
            values,
        }
    }
}

impl GqlEnum {
    pub fn contains(&self, name: &str) -> bool {
        let values: Vec<String> = self.values.iter().map(|v| v.name.clone()).collect();
        values.contains(&name.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct GqlEnumValue {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
}

impl<'a> From<EnumValue<'a, String>> for GqlEnumValue {
    fn from(enum_value: EnumValue<'a, String>) -> Self {
        let directives = enum_value
            .directives
            .into_iter()
            .map(|dir| GqlDirective::from(dir))
            .collect();

        GqlEnumValue {
            name: enum_value.name,
            description: enum_value.description,
            position: enum_value.position,
            directives,
        }
    }
}
