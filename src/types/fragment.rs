use graphql_parser::query::FragmentDefinition;

use super::GraphQLDirective;

pub struct GraphQLFragmentDefinition {
    name: String,
    directives: Vec<GraphQLDirective>,
}

impl GraphQLFragmentDefinition {
    pub fn parse<'a>(input: FragmentDefinition<'a, &'a str>) -> Self {
        let name = input.name.into();
        let directives: Vec<GraphQLDirective> = input
            .directives
            .into_iter()
            .map(|dir| GraphQLDirective::parse(dir))
            .collect();
        GraphQLFragmentDefinition { name, directives }
    }
}
