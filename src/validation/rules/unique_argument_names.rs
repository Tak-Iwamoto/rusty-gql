use std::collections::HashSet;

use graphql_parser::schema::Value;

use crate::validation::visitor::Visitor;

#[derive(Default)]
pub struct UniqueArgumentNames<'a> {
    names: HashSet<&'a str>,
}

impl<'a> Visitor<'a> for UniqueArgumentNames<'a> {
    fn enter_directive(
        &mut self,
        _ctx: &mut crate::validation::visitor::ValidationContext,
        _directive: &'a graphql_parser::schema::Directive<'a, String>,
    ) {
        self.names.clear();
    }

    fn enter_field(
        &mut self,
        _ctx: &mut crate::validation::visitor::ValidationContext,
        _field: &'a graphql_parser::query::Field<'a, String>,
    ) {
        self.names.clear();
    }

    fn enter_argument(
        &mut self,
        ctx: &mut crate::validation::visitor::ValidationContext,
        arg_name: &'a str,
        _arg_value: &'a Value<'a, String>,
    ) {
        if !self.names.insert(arg_name) {
            ctx.add_error(format!("{} is already contained.", arg_name), vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::test_utils::{
        assert_fails_rule, assert_passes_rule, check_fails_rule, check_passes_rule,
        get_query_fragment_definitions, parse_test_query, test_schema,
    };

    use super::UniqueArgumentNames;

    fn factory<'a>() -> UniqueArgumentNames<'a> {
        UniqueArgumentNames::default()
    }

    #[test]
    fn no_args_on_field() {
        let query_doc = r#"
        {
            human {
                name
            }
        }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn no_args_on_directive() {
        let query_doc = r#"
        {
            human {
                name @deprecated
            }
        }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn args_on_field() {
        let query_doc = r#"
        {
            droid(id: 1) {
                name
            }
        }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn args_on_directive() {
        let query_doc = r#"
        {
            human {
                name @skip(if: true)
            }
        }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn duplicate_args_on_field() {
        let query_doc = r#"
        {
            droid(id: 1, id: 2, id: 3) {
                name
            }
        }
        "#;
        check_fails_rule(query_doc, factory);
    }

    #[test]
    fn duplicate_args_on_directive() {
        let query_doc = r#"
        {
            human {
                name @skip(if: true, if: false, if: true)
            }
        }
        "#;
        check_fails_rule(query_doc, factory); }
}
