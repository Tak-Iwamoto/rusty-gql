use graphql_parser::query::Field;

use crate::{
    validation::{
        utils::{get_field_by_name, is_leaf_type, type_name_from_def},
        visitor::{ValidationContext, Visitor},
    },
    GqlTypeDefinition,
};

#[derive(Default)]
pub struct ScalarLeafs;

impl<'a> Visitor<'a> for ScalarLeafs {
    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            let is_exist = ctx
                .schema
                .type_definitions
                .get(&type_name_from_def(parent_type))
                .is_some();

            if is_exist {
                if let Some(target_field) = get_field_by_name(parent_type, &field.name) {
                    let target = ctx.schema.type_definitions.get(&target_field.name);

                    if let Some(ty) = target {
                        if is_leaf_type(ty) && !field.selection_set.items.is_empty() {
                            ctx.add_error(
                        format!("Field {} must not have a selection items because type {} has no subfields", &field.name, ty.to_string()),
                        vec![field.position])
                        } else if !is_leaf_type(ty) && field.selection_set.items.is_empty() {
                            ctx.add_error(
                                format!("Field {} must have selection items", &field.name),
                                vec![field.position],
                            )
                        }
                    }
                }
            }
        }
    }
}
