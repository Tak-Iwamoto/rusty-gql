use graphql_parser::query::Field;

use crate::{
    validation::{
        utils::type_name_from_def,
        visitor::{ValidationContext, Visitor},
    },
    GqlTypeDefinition,
};

#[derive(Default)]
pub struct FieldsOnCorrectType;

impl<'a> Visitor<'a> for FieldsOnCorrectType {
    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            if matches!(
                parent_type,
                GqlTypeDefinition::Union(_) | GqlTypeDefinition::Interface(_)
            ) {
                if field.name == "__typename" {
                    return;
                }
            }

            if parent_type.get_field_by_name(&field.name).is_none() {
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
