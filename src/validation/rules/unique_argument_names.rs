use std::collections::HashSet;

use graphql_parser::schema::Value;

use crate::validation::visitor::Visitor;

#[derive(Default)]
pub struct UniqueArgumentNames<'a> {
    names: HashSet<&'a str>,
}

impl<'a> Visitor<'a> for UniqueArgumentNames<'a> {
    fn enter_directive(
        &mut self,
        _ctx: &mut crate::validation::visitor::ValidationContext,
        _directive: &'a graphql_parser::schema::Directive<'a, String>,
    ) {
        self.names.clear();
    }

    fn enter_field(
        &mut self,
        _ctx: &mut crate::validation::visitor::ValidationContext,
        _field: &'a graphql_parser::query::Field<'a, String>,
    ) {
        self.names.clear();
    }

    fn enter_argument(
        &mut self,
        ctx: &mut crate::validation::visitor::ValidationContext,
        arg_name: &'a str,
        _arg_value: &'a Value<'a, String>,
    ) {
        if !self.names.insert(arg_name) {
            ctx.add_error(format!("{} is already contained.", arg_name), vec![])
        }
    }
}
