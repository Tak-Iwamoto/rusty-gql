use graphql_parser::schema::Type;

use crate::validation::{utils::check_valid_input_value, visitor::Visitor};

#[derive(Default)]
pub struct DefaultValueOfCorrectType;

impl<'a> Visitor<'a> for DefaultValueOfCorrectType {
    fn enter_variable_definition(
        &mut self,
        ctx: &mut crate::validation::visitor::ValidationContext,
        variable_definition: &'a graphql_parser::query::VariableDefinition<'a, String>,
    ) {
        if let Some(value) = &variable_definition.default_value {
            if matches!(&variable_definition.var_type, Type::NonNullType(_)) {
                ctx.add_error(format!("{} has type {} and it can't have a default value because it is non null type.", &variable_definition.name, &variable_definition.var_type), vec![variable_definition.position]);
            } else if let Some(err_msg) =
                check_valid_input_value(&ctx.schema, &variable_definition.var_type, value)
            {
                ctx.add_error(
                    format!("Invalid default value: {}", err_msg),
                    vec![variable_definition.position],
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_fails_rule, check_passes_rule};

    use super::*;

    pub fn factory() -> DefaultValueOfCorrectType {
        DefaultValueOfCorrectType
    }

    #[test]
    fn nullable_vars_no_default_values() {
        let query_doc = r#"
        query NoDefaultValues($a: Int, $b: String, $c: ReviewInput) {
            hero {
                name
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn required_vars_no_default_values() {
        let query_doc = r#"
        query RequiredValues($a: Int!, $b: String!) {
            hero {
                name
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn nullable_vars_with_valid_default_values() {
        let query_doc = r#"
        query WithDefaultValues($a: Int = 1, $b: String = "value", $c: ReviewInput = { stars: 1 }) {
            hero {
                name
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn required_vars_with_default_value() {
        let query_doc = r#"
        query WithRequiredDefaultValues($a: Int! = 1, $b: String! = "value") {
            hero {
                name
            }
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn nullable_vars_with_invalid_default_values() {
        let query_doc = r#"
        query WithInvalidValues($a: Int = "value", $b: String = 4, $c: ReviewInput = "invalidInput") {
            hero {
                name
            }
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn input_object_without_required_field() {
        let query_doc = r#"
        query WithoutRequiredField($a: ReviewInput = { commentary: "value" }) {
            hero {
                name
            }
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn list_vars_with_invalid_value() {
        let query_doc = r#"
        query InvalidList($a: [String] = ["value", 1]) {
            hero {
                name
            }
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }
}
