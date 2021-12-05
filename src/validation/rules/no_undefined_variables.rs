use std::collections::{HashMap, HashSet};

use graphql_parser::{query::OperationDefinition, Pos};

use crate::validation::{
    utils::{get_operation_def_position, referenced_variables, Scope},
    visitor::{ValidationContext, Visitor},
};

pub struct NoUndefinedVariables<'a> {
    defined_variables: HashMap<Option<&'a str>, (Pos, HashSet<&'a str>)>,
    used_variables: HashMap<Scope<'a>, HashMap<&'a str, Pos>>,
    current_scope: Option<Scope<'a>>,
    fragment_spreads: HashMap<Scope<'a>, Vec<&'a str>>,
}

impl<'a> NoUndefinedVariables<'a> {
    fn get_undefined_vars(
        &'a self,
        scope: &Scope<'a>,
        defined_vars: &HashSet<&'a str>,
        undefined_vars: &mut Vec<(&'a str, Pos)>,
        visited: &mut HashSet<Scope<'a>>,
    ) {
        if visited.contains(scope) {
            return;
        }

        visited.insert(*scope);

        if let Some(used_vars) = self.used_variables.get(scope) {
            for (var, pos) in used_vars {
                if !defined_vars.contains(var) {
                    undefined_vars.push((*var, *pos));
                }
            }
        }

        if let Some(spreads) = self.fragment_spreads.get(scope) {
            for spread in spreads {
                self.get_undefined_vars(
                    &Scope::Fragment(spread),
                    defined_vars,
                    undefined_vars,
                    visited,
                );
            }
        }
    }
}

impl<'a> Visitor<'a> for NoUndefinedVariables<'a> {
    fn exit_document(
        &mut self,
        ctx: &mut ValidationContext<'a>,
        _doc: &'a graphql_parser::query::Document<'a, String>,
    ) {
        for (name, (ref var_def_pos, ref var_defs)) in &self.defined_variables {
            let mut undefined_vars = Vec::new();
            let mut visited = HashSet::new();
            self.get_undefined_vars(
                &Scope::Operation(*name),
                var_defs,
                &mut undefined_vars,
                &mut visited,
            );
            for (var, pos) in undefined_vars {
                if let Some(operation_name) = name {
                    ctx.add_error(
                        format!(
                            "Variable {} is not defined by operation {}",
                            var, operation_name
                        ),
                        vec![pos, *var_def_pos],
                    )
                } else {
                    ctx.add_error(format!("Variable {} is not defined.", var), vec![pos]);
                }
            }
        }
    }

    fn enter_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        name: Option<&'a str>,
        operation_definition: &'a OperationDefinition<'a, String>,
    ) {
        self.current_scope = Some(Scope::Operation(name));
        self.defined_variables.insert(
            name,
            (
                get_operation_def_position(&operation_definition),
                HashSet::new(),
            ),
        );
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
        if let Some(Scope::Operation(name)) = &self.current_scope {
            if let Some(&mut (_, ref mut vars)) = self.defined_variables.get_mut(name) {
                vars.insert(&variable_definition.name);
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
                .or_insert_with(HashMap::new)
                .extend(
                    referenced_variables(&arg_value)
                        .into_iter()
                        .map(|n| (n, Pos::default())),
                );
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
