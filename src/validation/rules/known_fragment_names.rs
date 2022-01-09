use graphql_parser::query::FragmentSpread;

use crate::validation::visitor::{ValidationContext, Visitor};

#[derive(Default)]
pub struct KnownFragmentName;

impl<'a> Visitor<'a> for KnownFragmentName {
    fn enter_fragment_spread(
        &mut self,
        ctx: &mut ValidationContext,
        fragment_spread: &'a FragmentSpread<'a, String>,
    ) {
        if !ctx.fragments.contains_key(&fragment_spread.fragment_name) {
            ctx.add_error(
                format!("{} is not known fragment", &fragment_spread.fragment_name),
                vec![fragment_spread.position],
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::test_utils::{
        assert_fails_rule, assert_passes_rule, check_fails_rule, check_passes_rule,
        get_query_fragment_definitions, parse_test_query, test_schema,
    };

    use super::KnownFragmentName;

    fn factory() -> KnownFragmentName {
        KnownFragmentName::default()
    }
    #[test]
    fn include_known_fragment() {
        let query_doc = r#"
        {
            hero {
                ...CharacterFragment1
                ... on Character {
                    ...CharacterFragment2
                }
            }
        }
        fragment CharacterFragment1 on Character {
            name
        }
        fragment CharacterFragment2 on Character {
            friends
        }
        "#;
        check_passes_rule(query_doc, factory);
    }

    #[test]
    fn include_unknown_fragment() {
        let query_doc = r#"
        {
            hero {
                ...CharacterFragment1
                ... on Character {
                    ...CharacterFragment2
                }
            }
        }
        "#;
        check_fails_rule(query_doc, factory);
    }
}
