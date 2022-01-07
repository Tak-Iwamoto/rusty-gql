use std::collections::{HashMap, HashSet};

use graphql_parser::{
    query::{
        Document, FragmentDefinition, FragmentSpread, OperationDefinition, VariableDefinition,
    },
    schema::{Type, Value},
    Pos,
};

use crate::{
    validation::{
        utils::{is_gql_sub_type, is_sub_type, Scope},
        visitor::{ValidationContext, Visitor},
    },
    GqlValueType,
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
                    if !is_gql_sub_type(var_type, &GqlValueType::from(var_def.var_type)) {
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
