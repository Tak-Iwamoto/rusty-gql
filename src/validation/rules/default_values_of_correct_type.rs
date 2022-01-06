use graphql_parser::schema::Type;

use crate::validation::{utils::check_valid_input_value, visitor::Visitor};

#[derive(Default)]
pub struct DefaultValueOfCorrectType;

impl<'a> Visitor<'a> for DefaultValueOfCorrectType {
    fn enter_variable_definition(
        &mut self,
        ctx: &mut crate::validation::visitor::ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        if let Some(value) = &variable_definition.default_value {
            if matches!(&variable_definition.var_type, Type::NonNullType(_)) {
                ctx.add_error(format!("{} has type {} and it can't have a default value because it is non null type.", &variable_definition.name, &variable_definition.var_type), vec![variable_definition.position]);
            } else if let Some(err_msg) =
                check_valid_input_value(&ctx.schema, &variable_definition.var_type, value)
            {
                ctx.add_error(
                    format!("Invalid default value: {}", err_msg),
                    vec![variable_definition.position],
                )
            }
        }
    }
}
