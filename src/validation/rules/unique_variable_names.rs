use std::collections::HashSet;

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct UniqueVariableNames<'a> {
    names: HashSet<&'a str>,
}

impl<'a> Visitor<'a> for UniqueVariableNames<'a> {
    fn enter_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        _name: Option<&'a str>,
        _operation_definition: &'a graphql_parser::query::OperationDefinition<'a, String>,
    ) {
        self.names.clear();
    }

    fn enter_variable_definition(
        &mut self,
        ctx: &mut ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        if !self.names.insert(&variable_definition.name) {
            ctx.add_error(
                format!("{} is already contained.", &variable_definition.name),
                vec![variable_definition.position],
            );
        }
    }
}
