use crate::{
    validation::{
        utils::is_input_type,
        visitor::{ValidationContext, Visitor},
    },
    GqlValueType,
};

#[derive(Default)]
pub struct VariablesAreInputTypes;

impl<'a> Visitor<'a> for VariablesAreInputTypes {
    fn enter_variable_definition(
        &mut self,
        ctx: &mut ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        let ty = ctx
            .schema
            .type_definitions
            .get(GqlValueType::from(variable_definition.var_type.clone()).name());

        if let Some(variable_type) = ty {
            if !is_input_type(variable_type) {
                ctx.add_error(
                    format!(
                        "Variable {} cannot be non-input type {}",
                        &variable_definition.name,
                        variable_type.to_string()
                    ),
                    vec![variable_definition.position],
                );
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

    use super::VariablesAreInputTypes;

    fn factory() -> VariablesAreInputTypes {
        VariablesAreInputTypes
    }

    #[test]
    fn valid_types() {
        let query_doc = r#"
        query Test($a: String, $b:[Int!]!, $c: ReviewInput) {
            test_vars(a: $a, b: $b, c: $c)
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn invalid_types() {
        let query_doc = r#"
        query Test($a: Human, $b:[SearchResult!]!, $c: Character) {
            test_vars(a: $a, b: $b, c: $c)
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }
}
