use std::collections::{HashMap, HashSet};

use graphql_parser::{
    query::{
        Document, FragmentDefinition, FragmentSpread, OperationDefinition, VariableDefinition,
    },
    schema::Value,
    Pos,
};

use crate::{
    validation::{
        utils::Scope,
        visitor::{ValidationContext, Visitor},
    },
    GqlValue, GqlValueType,
};

#[derive(Default)]
pub struct VariablesInAllowedPosition<'a> {
    current_scope: Option<Scope<'a>>,
    variable_usages: HashMap<Scope<'a>, Vec<(&'a str, Pos, GqlValueType)>>,
    variable_definitions: HashMap<Scope<'a>, Vec<&'a VariableDefinition<'a, String>>>,
    fragment_spreads: HashMap<Scope<'a>, HashSet<&'a str>>,
}

impl<'a> VariablesInAllowedPosition<'a> {
    fn collect_incorret_variables(
        &self,
        scope: &Scope<'a>,
        variable_defs: &[&VariableDefinition<'a, String>],
        ctx: &mut ValidationContext<'a>,
        visited: &mut HashSet<Scope<'a>>,
    ) {
        if visited.contains(scope) {
            return;
        }
        visited.insert(*scope);

        if let Some(usages) = self.variable_usages.get(scope) {
            for (var_name, usage_pos, var_type) in usages {
                if let Some(var_def) = variable_defs.iter().find(|def| def.name == *var_name) {
                    let default_value = var_def.default_value.clone().map(|v| GqlValue::from(v));
                    if !var_type.is_sub_type(
                        &GqlValueType::from(var_def.var_type.clone()),
                        &default_value,
                    ) {
                        ctx.add_error(
                            format!(
                                "Variable {} of type {} used in positon expecting type {}",
                                var_name,
                                // TODO: gqlのtypeの文字列に変換する
                                var_type.name(),
                                &var_def.var_type
                            ),
                            vec![var_def.position, *usage_pos],
                        )
                    }
                }
            }
        }

        if let Some(fragment_spreads) = self.fragment_spreads.get(scope) {
            for sp in fragment_spreads {
                self.collect_incorret_variables(&Scope::Fragment(sp), variable_defs, ctx, visited)
            }
        }
    }
}

impl<'a> Visitor<'a> for VariablesInAllowedPosition<'a> {
    fn exit_document(&mut self, ctx: &mut ValidationContext<'a>, _doc: &'a Document<'a, String>) {
        for (scope, variable_defs) in &self.variable_definitions {
            self.collect_incorret_variables(scope, variable_defs, ctx, &mut HashSet::new());
        }
    }

    fn enter_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        name: Option<&'a str>,
        _operation_definition: &'a OperationDefinition<'a, String>,
    ) {
        self.current_scope = Some(Scope::Operation(name))
    }

    fn enter_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        name: &'a str,
        _fragment_definition: &'a FragmentDefinition<'a, String>,
    ) {
        self.current_scope = Some(Scope::Fragment(name))
    }

    fn enter_variable_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        variable_definition: &'a VariableDefinition<'a, String>,
    ) {
        if let Some(scope) = &self.current_scope {
            self.variable_definitions
                .entry(*scope)
                .or_insert_with(Vec::new)
                .push(variable_definition)
        }
    }

    fn enter_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        fragment_spread: &'a FragmentSpread<'a, String>,
    ) {
        if let Some(scope) = &self.current_scope {
            self.fragment_spreads
                .entry(*scope)
                .or_insert_with(HashSet::new)
                .insert(&fragment_spread.fragment_name);
        }
    }

    fn enter_input_value(
        &mut self,
        _ctx: &mut ValidationContext,
        expected_type: &Option<GqlValueType>,
        value: &'a Value<'a, String>,
        pos: Pos,
    ) {
        if let Value::Variable(var_name) = value {
            if let Some(ty) = expected_type {
                if let Some(scope) = &self.current_scope {
                    self.variable_usages
                        .entry(*scope)
                        .or_insert_with(Vec::new)
                        .push((var_name, pos, ty.clone()))
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

    use super::VariablesInAllowedPosition;

    fn factory<'a>() -> VariablesInAllowedPosition<'a> {
        VariablesInAllowedPosition::default()
    }

    #[test]
    fn boolean_into_boolean() {
        let query_doc = r#"
        query Test($boolArg: Boolean) {
            test_bool(boolArg: $boolArg) {
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
    fn boolean_into_boolean_with_fragment() {
        let query_doc = r#"
        fragment Frag on ArgsTest {
            booleanArgField(booleanArg: $boolArg)
        }
        query Test($boolArg: Boolean) {
            argTest {
                ...Frag
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn non_null_boolean_into_boolean() {
        let query_doc = r#"
        query Test($boolArg: Boolean!) {
            argTest {
                booleanArgField(booleanArg: $boolArg)
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn non_nullboolean_into_boolean_with_fragment() {
        let query_doc = r#"
        fragment Frag on ArgsTest {
            booleanArgField(booleanArg: $boolArg)
        }
        query Test($boolArg: Boolean!) {
            argTest {
                ...Frag
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn nullable_int_with_default_into_non_null_int() {
        let query_doc = r#"
        query Test($intArg: Int = 1) {
            argTest {
                nonNullIntArgField(intArg: $intArg)
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn nullable_int_into_non_null_int() {
        let query_doc = r#"
        query Test($intVar: Int) {
            argTest {
                nonNullIntArgField(intArg: $intVar)
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn string_list_into_string_list() {
        let query_doc = r#"
        query Test($stringListVar: [String]) {
            argTest {
                stringListArgField(stringListArg: $stringListVar)
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn non_null_string_list_into_string_list() {
        let query_doc = r#"
        query Test($stringListVar: [String!]) {
            argTest {
                stringListArgField(stringListArg: $stringListVar)
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn string_into_string_list_item_pos() {
        let query_doc = r#"
        query Test($stringVar: String) {
            argTest {
                stringListArgField(stringListArg: [$stringVar])
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn input_type_into_input_type() {
        let query_doc = r#"
        query Test($inputVar: ArgTestInput) {
            argTest {
                inputArgField(inputArg: $inputVar)
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn input_type_into_input_type_field_pos() {
        let query_doc = r#"
        query Test($boolVar: Boolean = true) {
            argTest {
                inputArgField(inputArg: {nonNullBooleanField: $boolVar})
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn non_nullstring_into_string_list_item_pos() {
        let query_doc = r#"
        query Test($stringVar: String!) {
            argTest {
                stringListArgField(stringListArg: [$stringVar])
            }
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn non_null_boolean_into_non_null_boolean_in_directive() {
        let query_doc = r#"
        query Test($boolVar: Boolean!) {
            hero @skip(if: $boolVar)
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn nullable_boolean_with_default_into_non_null_boolean_in_directive() {
        let query_doc = r#"
        query Test($boolVar: Boolean = false) {
            hero @skip(if: $boolVar)
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_passes_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn nullable_boolean_into_non_null_boolean_in_directive() {
        let query_doc = r#"
        query Test($boolVar: Boolean) {
            hero @skip(if: $boolVar)
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }

    #[test]
    fn nullable_string_into_non_null_boolean_in_directive() {
        let query_doc = r#"
        query Test($strVar: String) {
            hero @skip(if: $strVar)
        }
        "#;
        let schema = &test_schema();
        let doc = &parse_test_query(query_doc);
        let fragments = &get_query_fragment_definitions(doc, schema);
        assert_fails_rule(doc, schema, fragments, factory);
    }
}
