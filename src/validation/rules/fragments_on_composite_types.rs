use graphql_parser::query::{FragmentDefinition, InlineFragment};

use crate::validation::{
    utils::{is_composite_type, type_name_from_def},
    visitor::{ValidationContext, Visitor},
};

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
            let type_name = type_name_from_def(current_type);
            let target_type = ctx.schema.type_definitions.get(&type_name);

            if let Some(ty) = target_type {
                if !is_composite_type(ty) {
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
            let type_name = type_name_from_def(current_type);
            let target_type = ctx.schema.type_definitions.get(&type_name);

            if let Some(ty) = target_type {
                if !is_composite_type(ty) {
                    ctx.add_error(
                        format!("Fragment {} cannot condition non composite type", type_name),
                        vec![inline_fragment.position],
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::test_utils::{
        assert_fails_rule, assert_passes_rule, get_query_fragment_definitions, parse_test_query,
        test_schema,
    };

    use super::FragmentsOnCompositeTypes;

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
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn interface() {
        let query_doc = r#"
        fragment interfaceFrg on Character {
            name
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
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
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
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
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn union() {
        let query_doc = r#"
        fragment unionFrg on SearchResult {
            __typename
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn not_on_scalar() {
        let query_doc = r#"
        fragment scalarFrg on Boolean {
            invalid
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn not_on_enum() {
        let query_doc = r#"
        fragment enumFrg on LengthUnit {
            invalid
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn not_on_input_object() {
        let query_doc = r#"
        fragment enumFrg on ReviewInput {
            invalid
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
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
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }
}
