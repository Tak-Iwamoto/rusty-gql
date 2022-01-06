use crate::validation::{
    utils::is_input_type,
    visitor::{ValidationContext, Visitor},
};

#[derive(Default)]
pub struct VariablesAreInputTypes;

impl<'a> Visitor<'a> for VariablesAreInputTypes {
    fn enter_variable_definition(
        &mut self,
        ctx: &mut ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        let ty = ctx.schema.type_definitions.get(&variable_definition.name);

        if let Some(variable_type) = ty {
            if !is_input_type(variable_type) {
                ctx.add_error(
                    format!(
                        "Variable {} cannot be non-input type {}",
                        &variable_definition.name,
                        variable_type.to_string()
                    ),
                    vec![variable_definition.position],
                );
            }
        }
    }
}
