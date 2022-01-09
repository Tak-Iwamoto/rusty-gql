use graphql_parser::{
    query::Field,
    schema::{Directive, Value},
};

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct KnownArgumentNames<'a> {
    current_args: Option<(Vec<String>, ArgsPosition<'a>)>,
}

#[derive(Debug)]
enum ArgsPosition<'a> {
    Directive(&'a str),
    Field {
        field_name: &'a str,
        type_name: String,
    },
}

impl<'a> Visitor<'a> for KnownArgumentNames<'a> {
    fn enter_directive(
        &mut self,
        ctx: &mut ValidationContext,
        directive: &'a Directive<'a, String>,
    ) {
        if let Some(schema_directive) = ctx.schema.directives.get(&directive.name) {
            self.current_args = Some((
                schema_directive
                    .arguments
                    .iter()
                    .map(|arg| arg.name.clone())
                    .collect(),
                ArgsPosition::Directive(&directive.name),
            ));
        }
    }
    fn exit_directive(
        &mut self,
        _ctx: &mut ValidationContext,
        _directive: &'a Directive<'a, String>,
    ) {
        self.current_args = None;
    }

    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            if let Some(target_field) = parent_type.get_field_by_name(&field.name) {
                self.current_args = Some((
                    target_field
                        .arguments
                        .iter()
                        .map(|arg| arg.name.clone())
                        .collect(),
                    ArgsPosition::Field {
                        field_name: &field.name,
                        type_name: parent_type.name().to_string(),
                    },
                ))
            }
        }
    }

    fn exit_field(&mut self, _ctx: &mut ValidationContext, _field: &'a Field<'a, String>) {
        self.current_args = None
    }

    fn enter_argument(
        &mut self,
        ctx: &mut ValidationContext,
        arg_name: &'a str,
        _arg_value: &'a Value<'a, String>,
    ) {
        if let Some((args, arg_position)) = &self.current_args {
            if !args.iter().any(|arg| arg == arg_name) {
                match arg_position {
                    ArgsPosition::Directive(directive_name) => ctx.add_error(
                        format!(
                            "Unknown argument \"{}\" on directive \"{}\"",
                            arg_name, directive_name
                        ),
                        vec![],
                    ),
                    ArgsPosition::Field {
                        field_name,
                        type_name,
                    } => ctx.add_error(
                        format!(
                            "Unknown argument \"{}\" on field \"{}\" of type \"{}\"",
                            arg_name, field_name, type_name
                        ),
                        vec![],
                    ),
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

    use super::KnownArgumentNames;

    fn factory<'a>() -> KnownArgumentNames<'a> {
        KnownArgumentNames::default()
    }

    #[test]
    fn include_single_known_arguments() {
        let query_doc = r#"
        {
            droid(id: 1) {
                name
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn include_multiple_known_arguments() {
        let query_doc = r#"
        {
            search(text: "value", episode: "") {
                name
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn include_multiple_known_arguments_when_reverse_order() {
        let query_doc = r#"
        {
            search(episode: "", text: "value") {
                name
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn no_arguments_when_optional() {
        let query_doc = r#"
        {
            hero {
                friendsConnection {
                    totalCount
                }
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn invalid_argument_name() {
        let query_doc = r#"
        {
            hero {
                friendsConnection(invalid: "value") {
                    totalCount
                }
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn known_directive_args() {
        let query_doc = r#"
        {
            hero @include(if: true) {
                name
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn unknown_directive_args() {
        let query_doc = r#"
        {
            hero @include(unknown: true) {
                name
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }
}
