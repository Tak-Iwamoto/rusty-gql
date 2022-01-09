use graphql_parser::query::Field;

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct ScalarLeafs;

impl<'a> Visitor<'a> for ScalarLeafs {
    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            if let Some(target_field) = parent_type.get_field_by_name(&field.name) {
                if let Some(ty) = ctx
                    .schema
                    .type_definitions
                    .get(target_field.meta_type.name())
                {
                    if ty.is_leaf_type() && !field.selection_set.items.is_empty() {
                        ctx.add_error(
                        format!("Field {} must not have a selection items because type {} has no subfields", &field.name, ty.to_string()),
                        vec![field.position])
                    } else if !ty.is_leaf_type() && field.selection_set.items.is_empty() {
                        ctx.add_error(
                            format!("Field {} must have selection items", &field.name),
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

    fn factory() -> ScalarLeafs {
        ScalarLeafs
    }

    #[test]
    fn valid_scalar_selection() {
        let query_doc = r#"
        fragment objectFieldSelection on Human {
            name
        }
        { __typename }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn missing_object_field() {
        let query_doc = r#"
        query MissingField {
            human
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn missing_interface_field() {
        let query_doc = r#"
        {
            human(id: 1) {
                friends
            }
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn invalid_scalar_selection_on_boolean() {
        let query_doc = r#"
        fragment invalidBooleanField on Human {
            isValid { test }
        }
        { __typename }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn invalid_scalar_selection_on_enum() {
        let query_doc = r#"
        fragment invalidEnumField on Human {
            episode { test }
        }
        { __typename }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn invalid_scalar_selection_on_directive() {
        let query_doc = r#"
        fragment invalidEnumField on Human {
            name @include(if: true) { test }
        }
        { __typename }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn invalid_scalar_selection_on_directive_and_args() {
        let query_doc = r#"
        fragment invalidEnumField on Human {
            boolTest(if: false) @include(if: true) { test }
        }
        { __typename }
        "#;
        check_fails_rule!(query_doc, factory);
    }
}
