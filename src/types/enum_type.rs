use super::directive::GqlDirective;
use graphql_parser::{
    schema::{EnumType as ParserEnumType, EnumValue},
    Pos,
};

#[derive(Debug, Clone)]
pub struct EnumType {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
    pub values: Vec<EnumTypeValue>,
}

impl<'a> From<ParserEnumType<'a, String>> for EnumType {
    fn from(gql_enum: ParserEnumType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(gql_enum.directives);

        let values = gql_enum
            .values
            .into_iter()
            .map(EnumTypeValue::from)
            .collect();

        EnumType {
            name: gql_enum.name,
            description: gql_enum.description,
            position: gql_enum.position,
            directives,
            values,
        }
    }
}

impl EnumType {
    pub fn contains(&self, name: &str) -> bool {
        self.values
            .iter()
            .map(|v| v.name.clone())
            .any(|x| x == *name)
    }
}

#[derive(Debug, Clone)]
pub struct EnumTypeValue {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
}

impl<'a> From<EnumValue<'a, String>> for EnumTypeValue {
    fn from(enum_value: EnumValue<'a, String>) -> Self {
        let directives = enum_value
            .directives
            .into_iter()
            .map(GqlDirective::from)
            .collect();

        EnumTypeValue {
            name: enum_value.name,
            description: enum_value.description,
            position: enum_value.position,
            directives,
        }
    }
}

impl EnumTypeValue {
    pub fn is_deprecated(&self) -> bool {
        for dir in &self.directives {
            if dir.name == "deprecated" {
                return true;
            }
            continue;
        }
        false
    }
}
