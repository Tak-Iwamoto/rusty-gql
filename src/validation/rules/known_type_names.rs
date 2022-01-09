use graphql_parser::Pos;

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct KnownTypeNames;

impl<'a> Visitor<'a> for KnownTypeNames {
    fn enter_fragment_definition(
        &mut self,
        ctx: &mut ValidationContext,
        _name: &'a str,
        fragment_definition: &'a graphql_parser::query::FragmentDefinition<'a, String>,
    ) {
        match &fragment_definition.type_condition {
            graphql_parser::query::TypeCondition::On(on_ty) => {
                validate(ctx, on_ty, fragment_definition.position)
            }
        }
    }
    fn enter_variable_definition(
        &mut self,
        ctx: &mut ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        validate(
            ctx,
            &variable_definition.var_type.to_string(),
            variable_definition.position,
        )
    }

    fn enter_inline_fragment(
        &mut self,
        ctx: &mut ValidationContext,
        fragment_spread: &'a graphql_parser::query::InlineFragment<'a, String>,
    ) {
        if let Some(ty_condition) = &fragment_spread.type_condition {
            match ty_condition {
                graphql_parser::query::TypeCondition::On(on_ty) => {
                    validate(ctx, on_ty, fragment_spread.position)
                }
            }
        }
    }
}

fn validate(ctx: &mut ValidationContext, name: &str, position: Pos) {
    if !ctx.schema.type_definitions.contains_key(name) {
        ctx.add_error(format!("Unknown type {}", name), vec![position])
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_fails_rule, check_passes_rule};

    use super::*;

    fn factory() -> KnownTypeNames {
        KnownTypeNames::default()
    }

    #[test]
    fn include_known_types() {
        let query_doc = r#"
        query {
            hero {
                ...CharacterField
            }
            droid(id: 1) {
                ...CharacterField
            }
        }
        fragment CharacterField on Character {
            name
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn include_unknown_types() {
        let query_doc = r#"
        query {
            hero {
                ...CharacterField
                friends {
                    ... on Test { name }
                }
            }
            droid(id: 1) {
                ...CharacterField
            }
        }
        fragment CharacterField on Characterrr {
            name
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }
}
