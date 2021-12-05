use crate::validation::visitor::{ValidationContext, Visitor};

pub struct VariablesAreInputTypes;

impl<'a> Visitor<'a> for VariablesAreInputTypes {
    fn enter_variable_definition(
        &mut self,
        ctx: &mut ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        let gql_type = ctx.schema.type_definitions.get(&variable_definition.name);

        if let Some(variable_type) = gql_type {
            if !variable_type.is_input_type() {
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
