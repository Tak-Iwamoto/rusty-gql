use graphql_parser::query::FragmentSpread;

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct KnownFragmentName;

impl<'a> Visitor<'a> for KnownFragmentName {
    fn enter_fragment_spread(
        &mut self,
        ctx: &mut ValidationContext,
        fragment_spread: &'a FragmentSpread<'a, String>,
    ) {
        if !ctx.fragments.contains_key(&fragment_spread.fragment_name) {
            ctx.add_error(
                format!("{} is not known fragment", &fragment_spread.fragment_name),
                vec![fragment_spread.position],
            )
        }
    }
}
