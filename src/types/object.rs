use super::{field::GraphQLField, interface::GraphQLInterface, GraphQLDirective};

#[derive(Debug)]
pub struct GraphQLObject {
    name: String,
    description: Option<String>,
    fields: Vec<GraphQLField>,
    directives: Vec<GraphQLDirective>,
    interfaces: Vec<GraphQLInterface>,
}
