use graphql_parser::{
    query::{
        Definition, Document, Field, FragmentDefinition, FragmentSpread, InlineFragment,
        OperationDefinition, Selection, SelectionSet, VariableDefinition,
    },
    schema::{Directive, Value},
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
    fn enter_document(&mut self, _ctx: &mut ValidationContext<'a>, _doc: &'a Document<'a, String>) {
    }
    fn exit_document(&mut self, _ctx: &mut ValidationContext<'a>, _doc: &'a Document<'a, String>) {}
    fn enter_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        _operation_definition: &'a OperationDefinition<'a, String>,
    ) {
    }
    fn exit_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        _operation_definition: &'a OperationDefinition<'a, String>,
    ) {
    }

    fn enter_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _name: &'a str,
        _fragment_definition: &'a FragmentDefinition<'a, String>,
    ) {
    }
    fn exit_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _name: &'a str,
        _fragment_definition: &'a FragmentDefinition<'a, String>,
    ) {
    }

    fn enter_selection_set(
        &mut self,
        _ctx: &mut ValidationContext,
        _selection_set: &'a SelectionSet<'a, String>,
    ) {
    }
    fn exit_selection_set(
        &mut self,
        _ctx: &mut ValidationContext,
        _selection_set: &'a SelectionSet<'a, String>,
    ) {
    }

    fn enter_selection(
        &mut self,
        _ctx: &mut ValidationContext,
        _selection: &'a Selection<'a, String>,
    ) {
    }

    fn exit_selection(
        &mut self,
        _ctx: &mut ValidationContext,
        _selection: &'a Selection<'a, String>,
    ) {
    }
    fn enter_directive(
        &mut self,
        _ctx: &mut ValidationContext,
        _directive: &'a Directive<'a, String>,
    ) {
    }
    fn exit_directive(
        &mut self,
        _ctx: &mut ValidationContext,
        _directive: &'a Directive<'a, String>,
    ) {
    }

    fn enter_field(&mut self, _ctx: &mut ValidationContext, _field: &'a Field<'a, String>) {}
    fn exit_field(&mut self, _ctx: &mut ValidationContext, _field: &'a Field<'a, String>) {}

    fn enter_variable_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _variable_definition: &'a VariableDefinition<'a, String>,
    ) {
    }

    fn exit_variable_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _variable_definition: &'a VariableDefinition<'a, String>,
    ) {
    }

    fn enter_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_spread: &'a FragmentSpread<'a, String>,
    ) {
    }

    fn exit_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_spread: &'a FragmentSpread<'a, String>,
    ) {
    }

    fn enter_inline_fragment(
        &mut self,
        _ctx: &mut ValidationContext,
        _inline_fragment: &'a InlineFragment<'a, String>,
    ) {
    }

    fn exit_inline_fragment(
        &mut self,
        _ctx: &mut ValidationContext,
        _inline_fragment: &'a InlineFragment<'a, String>,
    ) {
    }

    fn enter_argument(
        &mut self,
        _ctx: &mut ValidationContext,
        _arg_name: &str,
        _arg_value: &'a Value<'a, String>,
    ) {
    }
    fn exit_argument(
        &mut self,
        _ctx: &mut ValidationContext,
        _arg_name: &str,
        _arg_value: &'a Value<'a, String>,
    ) {
    }
}

pub fn visit<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    doc: &'a Document<'a, String>,
) {
    visitor.enter_document(ctx, doc);
    visit_definitions(visitor, ctx, &doc.definitions);
    visitor.exit_document(ctx, doc);
}

fn visit_definitions<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    definitions: &'a [Definition<'a, String>],
) {
    for def in definitions {
        match def {
            Definition::Operation(operation) => {}
            Definition::Fragment(_) => todo!(),
        }
    }
}

fn visit_definition<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    definition: &'a Definition<'a, String>,
) {
    match definition {
        Definition::Operation(op) => visit_operation_definition(visitor, ctx, op),
        Definition::Fragment(fragment_def) => {
            visitor.enter_fragment_definition(ctx, &fragment_def.name, fragment_def)
        }
    }
}

fn visit_operation_definition<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    operation_definition: &'a OperationDefinition<'a, String>,
) {
    visitor.enter_operation_definition(ctx, operation_definition);
    visitor.exit_operation_definition(ctx, operation_definition);
}

fn visit_selection_set<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    selection_set: &'a SelectionSet<'a, String>,
) {
    if !selection_set.items.is_empty() {
        visitor.enter_selection_set(ctx, selection_set);
        for selection in &selection_set.items {}
        visitor.exit_selection_set(ctx, selection_set);
    }
}

fn visit_selection<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    selection: &'a Selection<'a, String>,
) {
    visitor.enter_selection(ctx, selection);
    match selection {
        Selection::Field(field) => if field.name == "__typename" {},
        Selection::FragmentSpread(fragment_spread) => {}
        Selection::InlineFragment(inline_fragment) => {}
    }
}

fn visit_field<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    field: &'a Field<'a, String>,
) {
    visitor.enter_field(ctx, field);

    for (arg_name, arg_value) in &field.arguments {
        visitor.enter_argument(ctx, arg_name, arg_value);
        visitor.exit_argument(ctx, arg_name, arg_value);
    }
    visitor.exit_field(ctx, field);
}

fn visit_fragment_definition<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    name: &'a str,
    fragment_definition: &'a FragmentDefinition<'a, String>,
) {
    visitor.enter_fragment_definition(ctx, name, fragment_definition);
    visit_directives(visitor, ctx, &fragment_definition.directives);
    visit_selection_set(visitor, ctx, &fragment_definition.selection_set);
    visitor.exit_fragment_definition(ctx, name, fragment_definition);
}

fn visit_fragment_spread<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    fragment_spread: &'a FragmentSpread<'a, String>,
) {
    visitor.enter_fragment_spread(ctx, fragment_spread);
    visit_directives(visitor, ctx, &fragment_spread.directives);
    visitor.exit_fragment_spread(ctx, fragment_spread);
}

fn visit_inline_fragment<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    inline_fragment: &'a InlineFragment<'a, String>,
) {
    visitor.enter_inline_fragment(ctx, inline_fragment);
    visit_directives(visitor, ctx, &inline_fragment.directives);
    visit_selection_set(visitor, ctx, &inline_fragment.selection_set);
    visitor.exit_inline_fragment(ctx, inline_fragment);
}

fn visit_directives<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    directives: &'a [Directive<'a, String>],
) {
    for directive in directives {
        visitor.enter_directive(ctx, directive);

        for (arg_name, arg_value) in &directive.arguments {
            visitor.enter_argument(ctx, arg_name, arg_value);
            visitor.exit_argument(ctx, arg_name, arg_value);
        }
        visitor.exit_directive(ctx, directive);
    }
}

fn exit_definition<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    definition: &'a Definition<'a, String>,
) {
    match definition {
        Definition::Operation(op) => visitor.exit_operation_definition(ctx, op),
        Definition::Fragment(fragment_def) => {
            visitor.exit_fragment_definition(ctx, &fragment_def.name, fragment_def)
        }
    }
}

fn visit_variable_definitions<'a, T: Visitor<'a>>(
    visitor: &mut T,
    ctx: &mut ValidationContext<'a>,
    variable_definitions: &'a Option<VariableDefinition<'a, String>>,
) {
}
