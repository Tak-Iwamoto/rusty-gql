use super::{GraphQLDirective, argument::GraphQLArgument, gql_type::GraphQLType};

#[derive(Debug)]
pub struct GraphQLQuery {
    pub name: String,
    pub args: Vec<GraphQLArgument>,
    pub description: Option<String>,
    pub directives: Vec<GraphQLDirective>,
    pub return_type: GraphQLType,
}

impl Default for GraphQLQuery {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            args: vec![],
            directives: vec![],
            return_type: GraphQLType::Null,
        }
    }
}
