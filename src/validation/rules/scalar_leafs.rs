use graphql_parser::query::Field;

use crate::validation::visitor::{Visitor, ValidationContext};

pub struct ScalarLeafs;

impl<'a> Visitor<'a> for ScalarLeafs {
    fn enter_field(
        &mut self,
        ctx: &mut ValidationContext,
        field: &'a Field<'a, String>,
    ) {
    }
}
