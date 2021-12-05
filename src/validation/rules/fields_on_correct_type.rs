use graphql_parser::query::Field;

use crate::validation::{
    utils::get_type_name,
    visitor::{ValidationContext, Visitor},
};

pub struct FieldsOnCorrectType;

impl<'a> Visitor<'a> for FieldsOnCorrectType {
    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            let field_name = &field.name;
        }
    }
}
