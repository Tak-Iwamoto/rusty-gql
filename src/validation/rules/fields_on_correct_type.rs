use graphql_parser::query::Field;

use crate::validation::{visitor::{Visitor, ValidationContext}, utils::get_type_name};

pub struct FieldsOnCorrectType;

impl<'a> Visitor<'a> for FieldsOnCorrectType {
    fn enter_field(
        &mut self,
        ctx: &mut ValidationContext,
        field: &'a Field<'a, String>,
    ) {
        if let Some(parent_type) = ctx.parent_type() {
            let field_name = &field.name;
            let type_name = get_type_name(parent_type);

        }
    }
}
