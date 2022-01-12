use std::collections::HashMap;

use graphql_parser::query::{Definition, Document, FragmentSpread, TypeCondition};

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct PossibleFragmentSpreads<'a> {
    fragment_types: HashMap<&'a str, &'a TypeCondition<'a, String>>,
}

impl<'a> Visitor<'a> for PossibleFragmentSpreads<'a> {
    fn enter_document(&mut self, _ctx: &mut ValidationContext<'a>, doc: &'a Document<'a, String>) {
        for def in &doc.definitions {
            if let Definition::Fragment(fragment) = def {
                self.fragment_types
                    .insert(&fragment.name, &fragment.type_condition);
            }
        }
    }

    fn enter_fragment_spread(
        &mut self,
        ctx: &mut ValidationContext,
        fragment_spread: &'a FragmentSpread<'a, String>,
    ) {
        if let Some(fragment_type) = self
            .fragment_types
            .get(&fragment_spread.fragment_name.as_str())
        {
            if let Some(_current_type) = ctx.current_type() {
                match fragment_type {
                    TypeCondition::On(on_type) => {
                        if let Some(_schema_on_type) = ctx.schema.type_definitions.get(on_type) {}
                    }
                }
            }
        }
    }
}
