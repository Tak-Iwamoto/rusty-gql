use std::collections::{HashMap, HashSet};

use graphql_parser::{
    query::{Definition, FragmentDefinition, FragmentSpread, InlineFragment, OperationDefinition},
    Pos,
};

use crate::validation::{
    utils::Scope,
    visitor::{ValidationContext, Visitor},
};

pub struct NoUnusedFragment<'a> {
    current_scope: Option<Scope<'a>>,
    fragment_spreads: HashMap<Scope<'a>, Vec<&'a str>>,
    fragment_definitions: HashSet<(&'a str, Pos)>,
}

impl<'a> NoUnusedFragment<'a> {
    fn get_reachable_fragments(&self, from: &Scope<'a>, result: &mut HashSet<&'a str>) {
        if let Scope::Fragment(name) = from {
            if result.contains(name) {
                return;
            } else {
                result.insert(name);
            }
        }

        if let Some(spreads) = self.fragment_spreads.get(from) {
            for spread in spreads {
                self.get_reachable_fragments(&Scope::Fragment(spread), result)
            }
        }
    }
}

impl<'a> Visitor<'a> for NoUnusedFragment<'a> {
    fn exit_document(
        &mut self,
        ctx: &mut ValidationContext<'a>,
        doc: &'a graphql_parser::query::Document<'a, String>,
    ) {
        let mut reachable = HashSet::new();

        for definition in &doc.definitions {
            if let Definition::Operation(operation) = definition {
                // let name = operation_name(&operation);
                // TODO: set operation name
                self.get_reachable_fragments(&Scope::Operation(None), &mut reachable)
            }
        }

        for fragment in &self.fragment_definitions {
            if !reachable.contains(&fragment.0) {
                ctx.add_error(
                    format!("{} is unused fragment.", &fragment.0),
                    vec![fragment.1],
                )
            }
        }
    }

    fn visit_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        operation_definition: &'a OperationDefinition<'a, String>,
    ) {
        // self.current_scope = Some(Scope::Operation(operation_definition.))
    }

    fn visit_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        fragment_definition: &'a FragmentDefinition<'a, String>,
    ) {
        self.current_scope = Some(Scope::Fragment(&fragment_definition.name));
        self.fragment_definitions
            .insert((&fragment_definition.name, fragment_definition.position));
    }

    fn visit_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        fragment_spread: &'a FragmentSpread<'a, String>,
    ) {
        if let Some(scope) = &self.current_scope {
            self.fragment_spreads
                .entry(scope.clone())
                .or_insert_with(Vec::new)
                .push(&fragment_spread.fragment_name)
        }
    }

    fn visit_inline_fragment(
        &mut self,
        _ctx: &mut ValidationContext,
        _inline_fragment: &'a InlineFragment<'a, String>,
    ) {
    }
}

fn operation_name<'a>(operation_definition: &'a OperationDefinition<'a, String>) -> Option<String> {
    match operation_definition {
        OperationDefinition::SelectionSet(_) => {
            // TODO: error handling
            unreachable!()
        }
        OperationDefinition::Query(query) => query.name.clone(),
        OperationDefinition::Mutation(mutation) => mutation.name.clone(),
        OperationDefinition::Subscription(subscription) => subscription.name.clone(),
    }
}
