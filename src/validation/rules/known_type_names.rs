use graphql_parser::Pos;

use crate::validation::visitor::{ValidationContext, Visitor};

pub struct KnownTypeNames {}

impl<'a> Visitor<'a> for KnownTypeNames {
    fn enter_fragment_definition(
        &mut self,
        ctx: &mut ValidationContext,
        fragment_definition: &'a graphql_parser::query::FragmentDefinition<'a, String>,
    ) {
        validate(ctx, &fragment_definition.name, fragment_definition.position)
    }

    fn enter_variable_definition(
        &mut self,
        ctx: &mut ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        validate(ctx, &variable_definition.name, variable_definition.position)
    }

    fn enter_inline_fragment(
        &mut self,
        ctx: &mut ValidationContext,
        fragment_spread: &'a graphql_parser::query::InlineFragment<'a, String>,
    ) {
        if let Some(ty_condition) = &fragment_spread.type_condition {
            match ty_condition {
                graphql_parser::query::TypeCondition::On(name) => {
                    validate(ctx, name, fragment_spread.position)
                }
            }
        }
    }
}

fn validate(ctx: &mut ValidationContext, name: &str, position: Pos) {
    if ctx.schema.type_map.get(name).is_none() {
        ctx.add_error(format!("Unknown type {}", name), vec![position])
    }
}
