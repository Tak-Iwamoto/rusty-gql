use std::collections::{HashMap, HashSet};

use graphql_parser::{
    query::{Definition, FragmentDefinition, FragmentSpread, OperationDefinition},
    Pos,
};

use crate::validation::{
    utils::Scope,
    visitor::{ValidationContext, Visitor},
};

#[derive(Default)]
pub struct NoUnusedFragment<'a> {
    current_scope: Option<Scope<'a>>,
    fragment_spreads: HashMap<Scope<'a>, Vec<&'a str>>,
    fragment_definitions: HashSet<(&'a str, Pos)>,
}

impl<'a> NoUnusedFragment<'a> {
    fn get_reachable_fragments(&self, from: &Scope<'a>, result: &mut HashSet<&'a str>) {
        if let Scope::Fragment(name) = from {
            if result.contains(name) {
                return;
            } else {
                result.insert(name);
            }
        }

        if let Some(spreads) = self.fragment_spreads.get(from) {
            for spread in spreads {
                self.get_reachable_fragments(&Scope::Fragment(spread), result)
            }
        }
    }
}

impl<'a> Visitor<'a> for NoUnusedFragment<'a> {
    fn exit_document(
        &mut self,
        ctx: &mut ValidationContext<'a>,
        doc: &'a graphql_parser::query::Document<'a, String>,
    ) {
        let mut reachable = HashSet::new();

        for definition in &doc.definitions {
            if let Definition::Operation(_) = definition {
                self.get_reachable_fragments(&Scope::Operation(None), &mut reachable)
            }
        }

        for fragment in &self.fragment_definitions {
            if !reachable.contains(&fragment.0) {
                ctx.add_error(
                    format!("{} is unused fragment.", &fragment.0),
                    vec![fragment.1],
                )
            }
        }
    }

    fn enter_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        name: Option<&'a str>,
        _operation_definition: &'a OperationDefinition<'a, String>,
    ) {
        self.current_scope = Some(Scope::Operation(name));
    }

    fn enter_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        name: &'a str,
        fragment_definition: &'a FragmentDefinition<'a, String>,
    ) {
        self.current_scope = Some(Scope::Fragment(name));
        self.fragment_definitions
            .insert((name, fragment_definition.position));
    }

    fn enter_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        fragment_spread: &'a FragmentSpread<'a, String>,
    ) {
        if let Some(scope) = &self.current_scope {
            self.fragment_spreads
                .entry(scope.clone())
                .or_insert_with(Vec::new)
                .push(&fragment_spread.fragment_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_fails_rule, check_passes_rule};

    use super::*;

    fn factory<'a>() -> NoUnusedFragment<'a> {
        NoUnusedFragment::default()
    }

    #[test]
    fn all_fragment_used() {
        let query_doc = r#"
        {
            hero {
                ...Frag1
                ... on Human {
                    ...Frag2
                }

            }
        }
        fragment Frag1 on Human {
            name
            ...Frag3
        }
        fragment Frag2 on Human {
            name
        }
        fragment Frag3 on Human {
            name
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn with_unused_fragment() {
        let query_doc = r#"
        {
            hero {
                ...Frag1
                ... on Human {
                    ...Frag2
                }

            }
        }
        fragment Frag1 on Human {
            name
            ...Frag3
        }
        fragment Frag2 on Human {
            name
        }
        fragment Frag3 on Human {
            name
        }
        fragment UnusedFrag1 on Human {
            name
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn with_unused_fragment_ref_cycle() {
        let query_doc = r#"
        {
            hero {
                ...Frag1
                ... on Human {
                    ...Frag2
                }

            }
        }
        fragment Frag1 on Human {
            name
            ...Frag3
        }
        fragment Frag2 on Human {
            name
        }
        fragment Frag3 on Human {
            name
        }
        fragment UnusedFrag1 on Human {
            name
            ...UnusedFrag2
        }
        fragment UnusedFrag2 on Human {
            name
            ...UnusedFrag1
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn with_unknown_and_unused_fragments() {
        let query_doc = r#"
        {
            hero {
                ...Frag1

            }
        }
        fragment UnusedFrag1 on Human {
            name
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }
}
