use graphql_parser::{schema::InputValue, Pos};

use super::{directive::GqlDirective, value::GqlValue, value_type::GqlValueType};

#[derive(Debug, Clone)]
pub struct ArgumentType {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub meta_type: GqlValueType,
    pub default_value: Option<GqlValue>,
    pub directives: Vec<GqlDirective>,
}

impl ArgumentType {
    pub fn from_vec_input_value<'a>(
        input_objects: Vec<InputValue<'a, String>>,
    ) -> Vec<ArgumentType> {
        input_objects
            .into_iter()
            .map(|arg| ArgumentType::from(arg))
            .collect()
    }
}

impl<'a> From<InputValue<'a, String>> for ArgumentType {
    fn from(input_value: InputValue<'a, String>) -> Self {
        let meta_type = GqlValueType::from(input_value.value_type);
        let default_value = input_value
            .default_value
            .map_or(None, |value| Some(GqlValue::from(value)));
        let directives = GqlDirective::from_vec_directive(input_value.directives);

        ArgumentType {
            name: input_value.name,
            description: input_value.description,
            position: input_value.position,
            meta_type,
            default_value,
            directives,
        }
    }
}
