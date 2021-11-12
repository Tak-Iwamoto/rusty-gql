use graphql_parser::{schema::InputValue, Pos};

use super::{directive::GqlDirective, meta_type::GqlMetaType, value::GqlValue};

#[derive(Debug)]
pub struct GqlArgument {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub meta_type: GqlMetaType,
    pub default_value: Option<GqlValue>,
    pub directives: Vec<GqlDirective>,
}

impl<'a> From<InputValue<'a, String>> for GqlArgument {
    fn from(input_value: InputValue<'a, String>) -> Self {
        let meta_type = GqlMetaType::from(input_value.value_type);
        let default_value = input_value
            .default_value
            .map_or(None, |value| Some(GqlValue::from(value)));
        let directives = input_value
            .directives
            .into_iter()
            .map(|dir| GqlDirective::from(dir))
            .collect();

        GqlArgument {
            name: input_value.name,
            description: input_value.description,
            position: input_value.position,
            meta_type,
            default_value,
            directives,
        }
    }
}
