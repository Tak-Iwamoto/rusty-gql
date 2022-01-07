use graphql_parser::{
    query::Field,
    schema::{Directive, Value},
};

use crate::validation::{
    utils::{get_field_by_name, type_name_from_def},
    visitor::{ValidationContext, Visitor},
};

#[derive(Default)]
pub struct KnownArgumentNames<'a> {
    current_args: Option<(Vec<String>, ArgsPosition<'a>)>,
}

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
        if ctx.schema.directives.get(&directive.name).is_some() {
            self.current_args = Some((
                directive
                    .arguments
                    .clone()
                    .into_iter()
                    .map(|arg| arg.0)
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
                        type_name: type_name_from_def(parent_type),
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
