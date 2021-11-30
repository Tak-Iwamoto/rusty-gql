use graphql_parser::{
    query::{
        Document, Field, FragmentDefinition, FragmentSpread, InlineFragment, OperationDefinition,
        Selection, SelectionSet, VariableDefinition,
    },
    schema::Directive,
    Pos,
};

use crate::Schema;

pub struct ValidationError {
    pub(crate) positions: Vec<Pos>,
    pub(crate) message: String,
}
pub struct ValidationContext<'a> {
    pub(crate) schema: &'a Schema,
    pub(crate) errors: Vec<ValidationError>,
}

impl<'a> ValidationContext<'a> {
    pub fn new(schema: &'a Schema) -> Self {
        ValidationContext {
            schema,
            errors: vec![],
        }
    }

    pub fn add_error<T: Into<String>>(&mut self, message: T, positions: Vec<Pos>) {
        self.errors.push(ValidationError {
            positions,
            message: message.into(),
        })
    }
}

pub trait Visitor<'a> {
    fn visit_document(&mut self, _ctx: &mut ValidationContext<'a>, _doc: &'a Document<'a, String>) {
    }
    fn visit_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        _operation_definition: &'a OperationDefinition<'a, String>,
    ) {
    }

    fn visit_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_definition: &'a FragmentDefinition<'a, String>,
    ) {
    }

    fn visit_selection_set(
        &mut self,
        _ctx: &mut ValidationContext,
        _selection_set: &'a SelectionSet<'a, String>,
    ) {
    }

    fn visit_selection(
        &mut self,
        _ctx: &mut ValidationContext,
        _selection_set: &'a Selection<'a, String>,
    ) {
    }

    fn visit_directive(
        &mut self,
        _ctx: &mut ValidationContext,
        _directive: &'a Directive<'a, String>,
    ) {
    }

    fn visit_field(&mut self, _ctx: &mut ValidationContext, _field: &'a Field<'a, String>) {}

    fn visit_variable_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _variable_definition: &'a VariableDefinition<'a, String>,
    ) {
    }

    fn visit_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_spread: &'a FragmentSpread<'a, String>,
    ) {
    }

    fn visit_inline_fragment(
        &mut self,
        _ctx: &mut ValidationContext,
        _inline_fragment: &'a InlineFragment<'a, String>,
    ) {
    }
}
