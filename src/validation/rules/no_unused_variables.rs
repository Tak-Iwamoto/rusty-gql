use std::collections::{HashMap, HashSet};

use graphql_parser::{query::Document, Pos};

use crate::validation::{
    utils::{referenced_variables, Scope},
    visitor::{ValidationContext, Visitor},
};

#[derive(Default)]
pub struct NoUnusedVariables<'a> {
    defined_variables: HashMap<Option<&'a str>, HashSet<(&'a str, Pos)>>,
    used_variables: HashMap<Scope<'a>, Vec<&'a str>>,
    current_scope: Option<Scope<'a>>,
    fragment_spreads: HashMap<Scope<'a>, Vec<&'a str>>,
}

impl<'a> NoUnusedVariables<'a> {
    fn get_used_vars(
        &self,
        scope: &Scope<'a>,
        defined_vars: &HashSet<&'a str>,
        used_vars: &mut HashSet<&'a str>,
        visited: &mut HashSet<Scope<'a>>,
    ) {
        if visited.contains(scope) {
            return;
        }
        visited.insert(*scope);

        if let Some(used_variables) = self.used_variables.get(scope) {
            for var in used_variables {
                if defined_vars.contains(var) {
                    used_vars.insert(var);
                }
            }
        }

        if let Some(fragment_spreads) = self.fragment_spreads.get(scope) {
            for sp in fragment_spreads {
                self.get_used_vars(&Scope::Fragment(sp), defined_vars, used_vars, visited)
            }
        }
    }
}

impl<'a> Visitor<'a> for NoUnusedVariables<'a> {
    fn exit_document(&mut self, ctx: &mut ValidationContext<'a>, _doc: &'a Document<'a, String>) {
        for (name, vars) in &self.defined_variables {
            let mut used_vars = HashSet::new();
            let mut visited = HashSet::new();
            self.get_used_vars(
                &Scope::Operation(*name),
                &vars.iter().map(|(name, _)| *name).collect(),
                &mut used_vars,
                &mut visited,
            );
            for (var, pos) in vars.iter().filter(|(var, _)| !used_vars.contains(var)) {
                if let Some(op_name) = name {
                    ctx.add_error(
                        format!("Variable {} is not used by operation {}", var, op_name),
                        vec![*pos],
                    );
                } else {
                    ctx.add_error(format!("Variable {} is not used", var), vec![*pos]);
                }
            }
        }
    }

    fn enter_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        name: Option<&'a str>,
        _operation_definition: &'a graphql_parser::query::OperationDefinition<'a, String>,
    ) {
        self.current_scope = Some(Scope::Operation(name));
        self.defined_variables.insert(name, HashSet::new());
    }

    fn enter_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        name: &'a str,
        _fragment_definition: &'a graphql_parser::query::FragmentDefinition<'a, String>,
    ) {
        self.current_scope = Some(Scope::Fragment(name));
    }

    fn enter_variable_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        if let Some(Scope::Operation(operation_name)) = &self.current_scope {
            if let Some(vars) = self.defined_variables.get_mut(operation_name) {
                vars.insert((&variable_definition.name, variable_definition.position));
            }
        }
    }

    fn enter_argument(
        &mut self,
        _ctx: &mut ValidationContext,
        _arg_name: &'a str,
        arg_value: &'a graphql_parser::schema::Value<'a, String>,
    ) {
        if let Some(scope) = &self.current_scope {
            self.used_variables
                .entry(*scope)
                .or_insert_with(Vec::new)
                .append(&mut referenced_variables(arg_value))
        }
    }

    fn enter_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        fragment_spread: &'a graphql_parser::query::FragmentSpread<'a, String>,
    ) {
        if let Some(scope) = &self.current_scope {
            self.fragment_spreads
                .entry(*scope)
                .or_insert_with(Vec::new)
                .push(&fragment_spread.fragment_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::test_utils::{
        assert_fails_rule, assert_passes_rule, get_query_fragment_definitions, parse_test_query,
        test_schema,
    };

    use super::NoUnusedVariables;

    fn factory<'a>() -> NoUnusedVariables<'a> {
        NoUnusedVariables::default()
    }

    #[test]
    fn uses_all_vars() {
        let query_doc = r#"
        query Test($a: String, $b: String, $c: String){
            test_vars(a: $a, b: $b, c: $c)
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory)
    }

    #[test]
    fn uses_all_vars_deeply() {
        let query_doc = r#"
        query Test($a: String, $b: String, $c: String){
            test_vars(a: $a) {
                test_vars(b: $b) {
                    test_vars(c: $c)
                }
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory)
    }

    #[test]
    fn uses_all_vars_deeply_in_inline_fragment() {
        let query_doc = r#"
        query Test($a: String, $b: String, $c: String){
            human {
                ... on Human {
                    test_vars(a: $a) {
                        test_vars(b: $b) {
                            test_vars(c: $c)
                        }
                    }
                }
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory)
    }

    #[test]
    fn uses_all_vars_deeply_in_fragments() {
        let query_doc = r#"
        query Test($a: String, $b: String, $c: String){
            human {
                ... on Human {
                    ...Frag1
                }
            }
        }
        fragment Frag1 on Human {
            test_vars(a: $a) {
                ...Frag2

            }
        }
        fragment Frag2 on Human {
            test_vars(b: $b) {
                ...Frag3
            }
        }
        fragment Frag3 on Human {
            test_vars(c: $c) {
                name
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory)
    }

    #[test]
    fn not_used_vars() {
        let query_doc = r#"
        query Test($a: String, $b: String, $c: String){
            test_vars(a: $a, b: $b)
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory)
    }

    #[test]
    fn not_used_multiple_vars() {
        let query_doc = r#"
        query Test($a: String, $b: String, $c: String){
            test_vars(a: $a)
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory)
    }

    #[test]
    fn not_used_in_fragments() {
        let query_doc = r#"
        query Test($a: String, $b: String, $c: String){
            human {
                ... on Human {
                    ...Frag1
                }
            }
        }
        fragment Frag1 on Human {
            test_vars(a: $a) {
                ...Frag2

            }
        }
        fragment Frag2 on Human {
            test_vars(b: $b) {
                ...Frag3
            }
        }
        fragment Frag3 on Human {
            name
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory)
    }

    #[test]
    fn not_used_in_unused_fragments() {
        let query_doc = r#"
        query Test($a: String, $b: String){
            human {
                ... on Human {
                    ...Frag1
                }
            }
        }
        fragment Frag1 on Human {
            test_vars(a: $a) {
                name
            }
        }
        fragment Frag2 on Human {
            test_vars(b: $b) {
                name
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory)
    }
}
