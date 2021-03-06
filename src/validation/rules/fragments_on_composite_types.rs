use graphql_parser::query::{FragmentDefinition, InlineFragment};

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct FragmentsOnCompositeTypes;

impl<'a> Visitor<'a> for FragmentsOnCompositeTypes {
    fn enter_fragment_definition(
        &mut self,
        ctx: &mut ValidationContext,
        name: &'a str,
        fragment_definition: &'a FragmentDefinition<'a, String>,
    ) {
        if let Some(current_type) = ctx.current_type() {
            let target_type = ctx.schema.type_definitions.get(current_type.name());

            if let Some(ty) = target_type {
                if !ty.is_composite_type() {
                    ctx.add_error(
                        format!("Fragment {} cannot condition non composite type", name),
                        vec![fragment_definition.position],
                    )
                }
            }
        }
    }

    fn enter_inline_fragment(
        &mut self,
        ctx: &mut ValidationContext,
        inline_fragment: &'a InlineFragment<'a, String>,
    ) {
        if let Some(current_type) = ctx.current_type() {
            let target_type = ctx.schema.type_definitions.get(current_type.name());

            if let Some(ty) = target_type {
                if !ty.is_composite_type() {
                    ctx.add_error(
                        format!(
                            "Fragment {} cannot condition non composite type",
                            current_type.name()
                        ),
                        vec![inline_fragment.position],
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_fails_rule, check_passes_rule};

    use super::*;

    fn factory() -> FragmentsOnCompositeTypes {
        FragmentsOnCompositeTypes::default()
    }

    #[test]
    fn object() {
        let query_doc = r#"
        fragment objectFrg on Human {
            name
        }
        { __typename }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn interface() {
        let query_doc = r#"
        fragment interfaceFrg on Character {
            name
        }
        { __typename }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn inline_object() {
        let query_doc = r#"
        fragment inlineFrg on Character {
            ... on Human {
                mass
            }
        }
        { __typename }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn inline_without_ty_cond() {
        let query_doc = r#"
        fragment withoutTy on Character {
            ... {
                name
            }
        }
        { __typename }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn union() {
        let query_doc = r#"
        fragment unionFrg on SearchResult {
            __typename
        }
        { __typename }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn not_on_scalar() {
        let query_doc = r#"
        fragment scalarFrg on Boolean {
            invalid
        }
        { __typename }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn not_on_enum() {
        let query_doc = r#"
        fragment enumFrg on LengthUnit {
            invalid
        }
        { __typename }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn not_on_input_object() {
        let query_doc = r#"
        fragment enumFrg on ReviewInput {
            invalid
        }
        { __typename }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn not_on_scalar_inline() {
        let query_doc = r#"
        fragment invalidFrg on Character {
            ... on String {
                name
            }
        }
        { __typename }
        "#;
        check_fails_rule!(query_doc, factory);
    }
}
