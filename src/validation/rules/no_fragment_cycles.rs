use std::collections::{HashMap, HashSet};

use graphql_parser::query::Document;

use crate::validation::visitor::{ValidationContext, ValidationError, Visitor};

struct CycleDetector<'a> {
    visited: HashSet<&'a str>,
    fragment_spreads: &'a HashMap<&'a str, Vec<&'a str>>,
    path_indices: HashMap<&'a str, usize>,
    errors: Vec<ValidationError>,
}

impl<'a> CycleDetector<'a> {
    fn detect_from(&mut self, from: &'a str, path: &mut Vec<&'a str>) {
        self.visited.insert(from);

        if !self.fragment_spreads.contains_key(from) {
            return;
        }

        self.path_indices.insert(from, path.len());

        for name in &self.fragment_spreads[from] {
            let index = self.path_indices.get(name).cloned();

            if index.is_some() {
                self.errors.push(ValidationError {
                    locations: vec![],
                    message: format!("Cannot spread fragment \"{}\"", name),
                })
            } else if !self.visited.contains(name) {
                path.push(name);
                self.detect_from(name, path);
                path.pop();
            }
        }

        self.path_indices.remove(from);
    }
}

#[derive(Default)]
pub struct NoFragmentCycles<'a> {
    current_fragment: Option<&'a str>,
    fragment_spreads: HashMap<&'a str, Vec<&'a str>>,
    fragment_order: Vec<&'a str>,
}

impl<'a> Visitor<'a> for NoFragmentCycles<'a> {
    fn exit_document(&mut self, ctx: &mut ValidationContext<'a>, _doc: &'a Document<'a, String>) {
        let mut detector = CycleDetector {
            visited: HashSet::new(),
            fragment_spreads: &self.fragment_spreads,
            path_indices: HashMap::new(),
            errors: Vec::new(),
        };

        for frag in &self.fragment_order {
            if !detector.visited.contains(frag) {
                let mut path = Vec::new();
                detector.detect_from(frag, &mut path);
            }
        }
        ctx.append_error(detector.errors);
    }

    fn enter_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        name: &'a str,
        _fragment_definition: &'a graphql_parser::query::FragmentDefinition<'a, String>,
    ) {
        self.current_fragment = Some(name);
        self.fragment_order.push(name);
    }

    fn exit_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _name: &'a str,
        _fragment_definition: &'a graphql_parser::query::FragmentDefinition<'a, String>,
    ) {
        self.current_fragment = None;
    }

    fn enter_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        fragment_spread: &'a graphql_parser::query::FragmentSpread<'a, String>,
    ) {
        if let Some(current_fragment) = self.current_fragment {
            self.fragment_spreads
                .entry(current_fragment)
                .or_insert_with(Vec::new)
                .push(&fragment_spread.fragment_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::test_utils::{
        assert_fails_rule, assert_passes_rule, check_fails_rule, check_passes_rule,
        get_query_fragment_definitions, parse_test_query, test_schema,
    };

    use super::NoFragmentCycles;

    fn factory<'a>() -> NoFragmentCycles<'a> {
        NoFragmentCycles::default()
    }

    #[test]
    fn valid_single_reference() {
        let query_doc = r#"
        fragment Frag1 on Human { ...Frag2 }
        fragment Frag2 on Human { name }
        { __typename }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn valid_twice_reference() {
        let query_doc = r#"
        fragment Frag1 on Human { ...Frag2 ...Frag2 }
        fragment Frag2 on Human { name }
        { __typename }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn twice_reference_is_not_circular() {
        let query_doc = r#"
        fragment Frag1 on Human { ...Frag2 ...Frag2 }
        fragment Frag2 on Human { ...Frag3 }
        fragment Frag3 on Human { name }
        { __typename }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn double_fragments_with_interface() {
        let query_doc = r#"
        fragment Frag1 on Character {
            ... on Human {
                name
            }
            ... on Droid {
                name
            }
        }
        fragment Frag2 on Character {
            ... on Human {
                ...Frag1
            }
            ... on Droid {
                ...Frag1
            }
        }
        { __typename }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn include_unknown_fragment() {
        let query_doc = r#"
        fragment Frag1 on Character {
            ...UnknownFragment
        }
        { __typename }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn spreading_itself() {
        let query_doc = r#"
        fragment Frag1 on Character {
            ...Frag1
        }
        { __typename }
        "#;
        check_fails_rule(query_doc, factory);
    }

    #[test]
    fn spreading_itself_in_inline_fragment() {
        let query_doc = r#"
        fragment Frag1 on Character {
            ... on Human {
                ...Frag1
            }
        }
        { __typename }
        "#;
        check_fails_rule(query_doc, factory);
    }
}
