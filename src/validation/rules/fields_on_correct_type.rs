use graphql_parser::{query::Field, schema::TypeDefinition};

use crate::validation::{
    utils::{get_field_by_name, type_name_from_def},
    visitor::{ValidationContext, Visitor},
};

#[derive(Default)]
pub struct FieldsOnCorrectType;

impl<'a> Visitor<'a> for FieldsOnCorrectType {
    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            if matches!(
                parent_type,
                TypeDefinition::Union(_) | TypeDefinition::Interface(_)
            ) {
                if field.name == "__typename" {
                    return;
                }
            }

            if get_field_by_name(&parent_type, &field.name).is_none() {
                ctx.add_error(
                    format!(
                        "Unknown field \"{}\" on type \"{}\"",
                        field.name,
                        type_name_from_def(parent_type)
                    ),
                    vec![field.position],
                )
            }
        }
    }
}
