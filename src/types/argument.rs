use graphql_parser::{schema::InputValue, Pos};

use super::{directive::GqlDirective, value::GqlValue, value_type::GqlValueType};

#[derive(Debug, Clone)]
pub struct InputValueType {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub meta_type: GqlValueType,
    pub default_value: Option<GqlValue>,
    pub directives: Vec<GqlDirective>,
}

impl InputValueType {
    pub fn from_vec_input_value(input_objects: Vec<InputValue<'_, String>>) -> Vec<InputValueType> {
        input_objects
            .into_iter()
            .map(InputValueType::from)
            .collect()
    }
}

impl<'a> From<InputValue<'a, String>> for InputValueType {
    fn from(input_value: InputValue<'a, String>) -> Self {
        let meta_type = GqlValueType::from(input_value.value_type);
        let default_value = input_value.default_value.map(GqlValue::from);
        let directives = GqlDirective::from_vec_directive(input_value.directives);

        InputValueType {
            name: input_value.name,
            description: input_value.description,
            position: input_value.position,
            meta_type,
            default_value,
            directives,
        }
    }
}
