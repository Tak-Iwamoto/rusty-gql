use graphql_parser::query::OperationDefinition;

use crate::validation::{
    utils::DirectiveLocation,
    visitor::{ValidationContext, Visitor},
};

#[derive(Default)]
pub struct KnownDirectives {
    location_stack: Vec<DirectiveLocation>,
}

impl<'a> Visitor<'a> for KnownDirectives {
    fn enter_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        _name: Option<&'a str>,
        operation_definition: &'a OperationDefinition<'a, String>,
    ) {
        self.location_stack.push(match operation_definition {
            OperationDefinition::Query(_) => DirectiveLocation::Query,
            OperationDefinition::Mutation(_) => DirectiveLocation::Mutation,
            OperationDefinition::Subscription(_) => DirectiveLocation::Subscription,
            OperationDefinition::SelectionSet(_) => return,
        })
    }

    fn exit_operation_definition(
        &mut self,
        _ctx: &mut ValidationContext<'a>,
        _name: Option<&'a str>,
        _operation_definition: &'a OperationDefinition<'a, String>,
    ) {
        self.location_stack.pop();
    }

    fn enter_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _name: &'a str,
        _fragment_definition: &'a graphql_parser::query::FragmentDefinition<'a, String>,
    ) {
        self.location_stack
            .push(DirectiveLocation::FragmentDefinition);
    }

    fn exit_fragment_definition(
        &mut self,
        _ctx: &mut ValidationContext,
        _name: &'a str,
        _fragment_definition: &'a graphql_parser::query::FragmentDefinition<'a, String>,
    ) {
        let top = self.location_stack.pop();
        assert_eq!(top, Some(DirectiveLocation::FragmentDefinition));
    }

    fn enter_directive(
        &mut self,
        ctx: &mut ValidationContext,
        directive: &'a graphql_parser::schema::Directive<'a, String>,
    ) {
        let is_exist = ctx.schema.directives.get(&directive.name).is_some();
        if !is_exist {
            ctx.add_error(
                format!("Unknown directive {}", directive.name),
                vec![directive.position],
            );
        }
    }

    fn enter_field(
        &mut self,
        _ctx: &mut ValidationContext,
        _field: &'a graphql_parser::query::Field<'a, String>,
    ) {
        self.location_stack.push(DirectiveLocation::Field);
    }

    fn exit_field(
        &mut self,
        _ctx: &mut ValidationContext,
        _field: &'a graphql_parser::query::Field<'a, String>,
    ) {
        let top = self.location_stack.pop();
        assert_eq!(top, Some(DirectiveLocation::Field));
    }

    fn enter_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_spread: &'a graphql_parser::query::FragmentSpread<'a, String>,
    ) {
        self.location_stack.push(DirectiveLocation::FragmentSpread);
    }

    fn exit_fragment_spread(
        &mut self,
        _ctx: &mut ValidationContext,
        _fragment_spread: &'a graphql_parser::query::FragmentSpread<'a, String>,
    ) {
        let top = self.location_stack.pop();
        assert_eq!(top, Some(DirectiveLocation::FragmentSpread));
    }

    fn enter_inline_fragment(
        &mut self,
        _ctx: &mut ValidationContext,
        _inline_fragment: &'a graphql_parser::query::InlineFragment<'a, String>,
    ) {
        self.location_stack.push(DirectiveLocation::InlineFragment);
    }

    fn exit_inline_fragment(
        &mut self,
        _ctx: &mut ValidationContext,
        _inline_fragment: &'a graphql_parser::query::InlineFragment<'a, String>,
    ) {
        let top = self.location_stack.pop();
        assert_eq!(top, Some(DirectiveLocation::InlineFragment));
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_fails_rule, check_passes_rule};

    use super::*;

    pub fn factory() -> KnownDirectives {
        KnownDirectives::default()
    }

    #[test]
    fn no_directives() {
        let query_doc = r#"
        query {
            hero {
                name
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn include_known_directive() {
        let query_doc = r#"
        {
            hero @include(if: true) {
                name
            }
            droid(id: 1) @skip(if: false) {
                name
            }
        }
        "#;
        check_passes_rule!(query_doc, factory);
    }

    #[test]
    fn include_unknow_directive() {
        let query_doc = r#"
        {
            hero @unknown {
                name
            }
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }

    #[test]
    fn include_multiple_unknown_directive() {
        let query_doc = r#"
        {
            hero @unknown {
                name
            }
            droid(id: 1) @unknown_dir(test: "value") {
                name
            }
        }
        "#;
        check_fails_rule!(query_doc, factory);
    }
}
