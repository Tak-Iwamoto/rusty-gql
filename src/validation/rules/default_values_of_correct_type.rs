use crate::validation::visitor::Visitor;

pub struct DefaultValueOfCorrectType;

impl<'a> Visitor<'a> for DefaultValueOfCorrectType {

    fn enter_variable_definition(
        &mut self,
        ctx: &mut crate::validation::visitor::ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        if let Some(value) = &variable_definition.default_value {

        }
    }

}
