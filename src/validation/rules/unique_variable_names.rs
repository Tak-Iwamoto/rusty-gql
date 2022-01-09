use std::collections::HashSet;

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct UniqueVariableNames<'a> {
    names: HashSet<&'a str>,
}

impl<'a> Visitor<'a> for UniqueVariableNames<'a> {
    fn enter_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        _name: Option<&'a str>,
        _operation_definition: &'a graphql_parser::query::OperationDefinition<'a, String>,
    ) {
        self.names.clear();
    }

    fn enter_variable_definition(
        &mut self,
        ctx: &mut ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        if !self.names.insert(&variable_definition.name) {
            ctx.add_error(
                format!("{} is already contained.", &variable_definition.name),
                vec![variable_definition.position],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::test_utils::{
        assert_fails_rule, assert_passes_rule, get_query_fragment_definitions, parse_test_query,
        test_schema,
    };

    use super::UniqueVariableNames;

    fn factory<'a>() -> UniqueVariableNames<'a> {
        UniqueVariableNames::default()
    }

    #[test]
    fn unique_var_names() {
        let query_doc = r#"
        query Test($a: Int, $b: String) {
            __typename
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn duplicate_var_names() {
        let query_doc = r#"
        query Test($a: Int, $a: String) {
            __typename
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }
}
