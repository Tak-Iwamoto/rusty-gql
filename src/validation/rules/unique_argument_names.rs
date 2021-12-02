use std::collections::HashSet;

use graphql_parser::schema::Value;

use crate::validation::visitor::Visitor;

pub struct UniqueArgumentNames<'a> {
    names: HashSet<&'a str>,
}

impl<'a> Visitor<'a> for UniqueArgumentNames<'a> {
    fn visit_directive(
        &mut self,
        _ctx: &mut crate::validation::visitor::ValidationContext,
        _directive: &'a graphql_parser::schema::Directive<'a, String>,
    ) {
        self.names.clear();
    }

    fn visit_field(
        &mut self,
        _ctx: &mut crate::validation::visitor::ValidationContext,
        _field: &'a graphql_parser::query::Field<'a, String>,
    ) {
        self.names.clear();
    }

    fn visit_argument(&mut self, _ctx: &mut crate::validation::visitor::ValidationContext, arg: &'a Value<'a, String>) {

    }
}
