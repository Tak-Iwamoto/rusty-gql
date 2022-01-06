use graphql_parser::schema::Value;

use crate::validation::{
    utils::check_valid_input_value,
    visitor::{ValidationContext, Visitor},
};

#[derive(Default)]
pub struct ArgumentsOfCorrectType<'a> {
    pub current_args: Option<&'a Vec<(String, Value<'a, String>)>>,
}

impl<'a> Visitor<'a> for ArgumentsOfCorrectType<'a> {
    fn enter_directive(
        &mut self,
        _ctx: &mut ValidationContext,
        directive: &'a graphql_parser::schema::Directive<'a, String>,
    ) {
        self.current_args = Some(&directive.arguments);
    }

    fn exit_directive(
        &mut self,
        _ctx: &mut ValidationContext,
        _directive: &'a graphql_parser::schema::Directive<'a, String>,
    ) {
        self.current_args = None;
    }

    fn enter_field(
        &mut self,
        _ctx: &mut ValidationContext,
        field: &'a graphql_parser::query::Field<'a, String>,
    ) {
        self.current_args = Some(&field.arguments);
    }

    fn exit_field(
        &mut self,
        _ctx: &mut ValidationContext,
        _field: &'a graphql_parser::query::Field<'a, String>,
    ) {
        self.current_args = None;
    }

    fn enter_argument(
        &mut self,
        ctx: &mut ValidationContext,
        arg_name: &'a str,
        arg_value: &'a Value<'a, String>,
    ) {
        match &self.current_args {
            Some(args) => {
                let target_arg = args.iter().find(|arg| arg.0 == arg_name);
                if target_arg.is_none() {
                    return;
                }

                // if let Some(vars) = &ctx.variables {
                //     if let Some(def) = vars.0.get(arg_name) {
                //         if let Some(err_msg) =
                //             check_valid_input_value(&ctx.schema, &def.var_type, arg_value)
                //         {
                //             ctx.add_error(format!("Invalid value for argument {}", err_msg), vec![])
                //         }
                //     }
                // }
            }
            None => return,
        }
    }
}
