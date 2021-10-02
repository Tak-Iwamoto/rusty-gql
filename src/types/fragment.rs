use super::GraphQLDirective;

pub struct GraphQLFragment {
    name: String,
    directives: Vec<GraphQLDirective>,
}
