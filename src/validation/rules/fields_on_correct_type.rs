use graphql_parser::query::Field;

use crate::{
    validation::{
        utils::type_name_from_def,
        visitor::{ValidationContext, Visitor},
    },
    GqlTypeDefinition,
};

#[derive(Default)]
pub struct FieldsOnCorrectType;

impl<'a> Visitor<'a> for FieldsOnCorrectType {
    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            if matches!(
                parent_type,
                GqlTypeDefinition::Union(_) | GqlTypeDefinition::Interface(_)
            ) {
                if field.name == "__typename" {
                    return;
                }
            }
            println!("{:?}", parent_type);

            if parent_type.get_field_by_name(&field.name).is_none() {
                ctx.add_error(
                    format!(
                        "Unknown field \"{}\" on type \"{}\"",
                        field.name,
                        type_name_from_def(parent_type)
                    ),
                    vec![field.position],
                )
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

    use super::FieldsOnCorrectType;

    pub fn factory() -> FieldsOnCorrectType {
        FieldsOnCorrectType::default()
    }

    #[test]
    fn object_selection() {
        let query_doc = r#"
        fragment objectFieldSelection on Human {
            __typename
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
    fn interface_unknown_field() {
        let query_doc = r#"
        fragment unknownField on Character {
            unknownField
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn nested_unknown_fields() {
        let query_doc = r#"
        fragment unknownField on Character {
            unknownField {
                ... on Human {
                    unknown_human_field
                }
            }
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn unknown_sub_fields() {
        let query_doc = r#"
        fragment unknownSubField on Character {
            friends {
                unknownField
            }
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn union_typename() {
        let query_doc = r#"
        fragment objectSelection on SearchResult {
            __typename
            ... on Human {
                name
            }
            ... on Droid {
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
    fn union_field_name() {
        let query_doc = r#"
        fragment objectSelection on SearchResult {
            name
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn union_meta_field() {
        let query_doc = r#"
        fragment objectSelection on SearchResult {
            __typename
        }
        { __typename }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

}
