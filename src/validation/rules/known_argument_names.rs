use graphql_parser::{
    query::Field,
    schema::{Directive, Value},
};

use crate::validation::visitor::{ValidationContext, Visitor};

pub struct KnownArgumentNames<'a> {
    // pub current_args: Option<(String, Value<'a, String>), ArgsPosition<'a>>
    current_args: Option<(&'a Vec<(String, Value<'a, String>)>, ArgsPosition<'a>)>,
}

enum ArgsPosition<'a> {
    Directive(&'a str),
    Field {
        field_name: &'a str,
        type_name: &'a str,
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
                &directive.arguments,
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
        // if let Some(parent_type) = ctx.parent_type() {
        //     if let Some(schema_field) = ctx.schema.type_map.get(&get_type_name(&parent_type)) {
        //         if schema_field.get_field_by_name(&field.name).is_some() {
        //             self.current_args = Some((
        //                 &field.arguments,
        //                 ArgsPosition::Field {
        //                     field_name: &field.name,
        //                     type_name: &get_type_name(&parent_type).clone(),
        //                 },
        //             ))
        //         }
        //     }
        // }
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
            if !args.iter().any(|arg| arg.0 == arg_name) {
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
