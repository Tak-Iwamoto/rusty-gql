use crate::{
    validation::visitor::{ValidationContext, Visitor},
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
            if !variable_type.is_input_type() {
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
    use crate::{check_fails_rule, check_passes_rule};

    use super::*;

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
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn invalid_types() {
        let query_doc = r#"
        query Test($a: Human, $b:[SearchResult!]!, $c: Character) {
            test_vars(a: $a, b: $b, c: $c)
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }
}
