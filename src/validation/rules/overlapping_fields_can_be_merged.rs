use std::collections::HashMap;

use graphql_parser::query::{Field, Selection, SelectionSet};

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct OverlappingFieldsCanBeMerged;

impl<'a> Visitor<'a> for OverlappingFieldsCanBeMerged {
    fn enter_selection_set(
        &mut self,
        ctx: &mut ValidationContext<'a>,
        selection_set: &'a SelectionSet<'a, String>,
    ) {
        let mut find_conflicts = FindConflicts {
            outputs: Default::default(),
        };
        find_conflicts.find(ctx, selection_set);
    }
}

struct FindConflicts<'a> {
    outputs: HashMap<&'a str, &'a Field<'a, String>>,
}

impl<'a> FindConflicts<'a> {
    pub fn find(
        &mut self,
        ctx: &mut ValidationContext<'a>,
        selection_set: &'a SelectionSet<'a, String>,
    ) {
        for item in &selection_set.items {
            match item {
                Selection::Field(field) => {
                    let name = match &field.alias {
                        Some(alias) => alias,
                        None => &field.name,
                    };
                    self.add_output(ctx, name, field);
                }
                Selection::FragmentSpread(spread) => {
                    if let Some(fragment) = ctx.fragments.get(&spread.fragment_name) {
                        self.find(ctx, &fragment.selection_set);
                    }
                }
                Selection::InlineFragment(inline_fragment) => {
                    self.find(ctx, &inline_fragment.selection_set)
                }
            }
        }
    }

    pub fn add_output(
        &mut self,
        ctx: &mut ValidationContext<'a>,
        name: &'a str,
        field: &'a Field<'a, String>,
    ) {
        match self.outputs.get(name) {
            Some(prev_field) => {
                if prev_field.name != field.name {
                    ctx.add_error(
                        format!(
                            "Fields {} conflict because {} and {} are different fields.",
                            name, prev_field.name, field.name
                        ),
                        vec![field.position, prev_field.position],
                    )
                }

                if prev_field.arguments.len() != field.arguments.len() {
                    ctx.add_error(
                        format!(
                            "Fields {} conflict because they have different arguments.",
                            name
                        ),
                        vec![field.position, prev_field.position],
                    )
                }

                for (arg_name, arg_value) in &prev_field.arguments {
                    match field.arguments.iter().find(|(name, _)| name == arg_name) {
                        Some(arg) => {
                            if &arg.1 == arg_value {
                                {}
                            }
                        }
                        None => ctx.add_error(
                            format!(
                                "Fields {} conflict because the have different arguments",
                                name
                            ),
                            vec![prev_field.position, field.position],
                        ),
                    }
                }
            }
            None => {
                self.outputs.insert(name, field);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OverlappingFieldsCanBeMerged;

    #[allow(unused)]
    fn factory() -> OverlappingFieldsCanBeMerged {
        OverlappingFieldsCanBeMerged
    }

    #[test]
    fn todo() {}
}
