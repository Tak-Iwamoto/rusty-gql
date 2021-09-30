use super::argument::GraphQLArgument;

pub struct GraphQLDirective {
    name: String,
    description: Option<String>,
    args: Vec<GraphQLArgument>,
}
