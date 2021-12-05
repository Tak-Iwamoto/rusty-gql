use graphql_parser::query::{Document, OperationDefinition};

use crate::validation::{
    utils::Scope,
    visitor::{ValidationContext, Visitor},
};

pub struct VariablesInAllowedPosition<'a> {
    current_scope: Option<Scope<'a>>,
}

impl<'a> Visitor<'a> for VariablesInAllowedPosition<'a> {
    fn exit_document(&mut self, _ctx: &mut ValidationContext<'a>, _doc: &'a Document<'a, String>) {}

    fn enter_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        _name: Option<&'a str>,
        operation_definition: &'a OperationDefinition<'a, String>,
    ) {
    }
}
