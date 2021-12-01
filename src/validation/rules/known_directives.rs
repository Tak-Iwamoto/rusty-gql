use graphql_parser::query::{OperationDefinition, SelectionSet};

use crate::validation::{
    utils::DirectiveLocation,
    visitor::{ValidationContext, Visitor},
};

pub struct KnownDirectives {
    location_stack: Vec<DirectiveLocation>,
}

impl<'a> Visitor<'a> for KnownDirectives {
    fn visit_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        operation_definition: &'a OperationDefinition<'a, String>,
    ) {
        self.location_stack.push(match operation_definition {
            OperationDefinition::Query(_) => DirectiveLocation::Query,
            OperationDefinition::Mutation(_) => DirectiveLocation::Mutation,
            OperationDefinition::Subscription(_) => DirectiveLocation::Subscription,
            OperationDefinition::SelectionSet(_) => return,
        })
    }

    fn exit_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        _operation_definition: &'a OperationDefinition<'a, String>,
    ) {
        let top = self.location_stack.pop();
        assert!(
            top == Some(DirectiveLocation::Query)
                || top == Some(DirectiveLocation::Mutation)
                || top == Some(DirectiveLocation::Subscription)
        );
    }

    fn visit_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_definition: &'a graphql_parser::query::FragmentDefinition<'a, String>,
    ) {
        self.location_stack
            .push(DirectiveLocation::FragmentDefinition);
    }

    fn exit_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_definition: &'a graphql_parser::query::FragmentDefinition<'a, String>,
    ) {
        let top = self.location_stack.pop();
        assert_eq!(top, Some(DirectiveLocation::FragmentDefinition));
    }

    fn visit_directive(
        &mut self,
        _ctx: &mut ValidationContext,
        directive: &'a graphql_parser::schema::Directive<'a, String>,
    ) {
    }

    fn visit_field(
        &mut self,
        _ctx: &mut ValidationContext,
        _field: &'a graphql_parser::query::Field<'a, String>,
    ) {
        self.location_stack.push(DirectiveLocation::Field);
    }

    fn exit_field(
        &mut self,
        _ctx: &mut ValidationContext,
        _field: &'a graphql_parser::query::Field<'a, String>,
    ) {
        let top = self.location_stack.pop();
        assert_eq!(top, Some(DirectiveLocation::Field));
    }

    fn visit_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_spread: &'a graphql_parser::query::FragmentSpread<'a, String>,
    ) {
        self.location_stack.push(DirectiveLocation::FragmentSpread);
    }

    fn exit_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_spread: &'a graphql_parser::query::FragmentSpread<'a, String>,
    ) {
        let top = self.location_stack.pop();
        assert_eq!(top, Some(DirectiveLocation::FragmentSpread));
    }

    fn visit_inline_fragment(
        &mut self,
        _ctx: &mut ValidationContext,
        _inline_fragment: &'a graphql_parser::query::InlineFragment<'a, String>,
    ) {
        self.location_stack.push(DirectiveLocation::InlineFragment);
    }

    fn exit_inline_fragment(
        &mut self,
        _ctx: &mut ValidationContext,
        _inline_fragment: &'a graphql_parser::query::InlineFragment<'a, String>,
    ) {
        let top = self.location_stack.pop();
        assert_eq!(top, Some(DirectiveLocation::InlineFragment));
    }
}
