use graphql_parser::query::Field;

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct ScalarLeafs;

impl<'a> Visitor<'a> for ScalarLeafs {
    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            let is_exist = ctx
                .schema
                .type_definitions
                .get(parent_type.name())
                .is_some();

            if is_exist {
                if let Some(target_field) = parent_type.get_field_by_name(&field.name) {
                    let target = ctx.schema.type_definitions.get(&target_field.name);

                    if let Some(ty) = target {
                        if ty.is_leaf_type() && !field.selection_set.items.is_empty() {
                            ctx.add_error(
                        format!("Field {} must not have a selection items because type {} has no subfields", &field.name, ty.to_string()),
                        vec![field.position])
                        } else if !ty.is_leaf_type() && field.selection_set.items.is_empty() {
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
