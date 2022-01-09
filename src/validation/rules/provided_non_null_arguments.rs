use graphql_parser::{query::Field, schema::Directive};

use crate::validation::{
    utils::get_type_name,
    visitor::{ValidationContext, Visitor},
};

#[derive(Default)]
pub struct ProvidedNonNullArguments;

impl<'a> Visitor<'a> for ProvidedNonNullArguments {
    fn enter_directive(
        &mut self,
        ctx: &mut ValidationContext,
        directive: &'a Directive<'a, String>,
    ) {
        if let Some(schema_directive) = ctx.schema.directives.get(&directive.name) {
            for arg in &schema_directive.arguments {
                if arg.meta_type.is_non_null()
                    && arg.default_value.is_none()
                    && !directive
                        .arguments
                        .iter()
                        .any(|(name, _)| name.eq(&arg.name))
                {
                    ctx.add_error(
                        format!(
                            "Directive @{} argument {} of type {} is required but not provided",
                            directive.name,
                            arg.name,
                            get_type_name(&arg.meta_type.to_parser_type())
                        ),
                        vec![directive.position],
                    )
                }
            }
        }
    }

    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            if let Some(target_field) = parent_type.get_field_by_name(&field.name) {
                for arg in &target_field.arguments {
                    if arg.meta_type.is_non_null()
                        && arg.default_value.is_none()
                        && !field.arguments.iter().any(|(name, _)| name.eq(&arg.name))
                    {
                        ctx.add_error(
                            format!(
                                "Field {} argument {} of type {} is required but not provided",
                                field.name,
                                arg.name,
                                parent_type.name(),
                            ),
                            vec![field.position],
                        )
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_fails_rule, check_passes_rule};

    use super::*;

    fn factory() -> ProvidedNonNullArguments {
        ProvidedNonNullArguments
    }

    #[test]
    fn ignore_unknown_args() {
        let query_doc = r#"
        {
            human(id: 1) {
                friendsConnection(unknown_arg: "value") {
                    name
                }
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn args_on_nullable_arg() {
        let query_doc = r#"
        {
            human(id: 1) {
                friendsConnection(first: 10) {
                    name
                }
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn no_args_on_nullable_arg() {
        let query_doc = r#"
        {
            human(id: 1) {
                friendsConnection {
                    name
                }
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn missing_non_null_arg() {
        let query_doc = r#"
        {
            human(id: 1) {
                non_null_test {
                    name
                }
            }
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn ignore_unknown_directives() {
        let query_doc = r#"
        {
            human(id: 1) @unknown {
                name
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn valid_directive() {
        let query_doc = r#"
        {
            human(id: 1) @skip(if: false) {
                name
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn missing_directive_arg() {
        let query_doc = r#"
        {
            human(id: 1) @skip {
                name
            }
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }
}
