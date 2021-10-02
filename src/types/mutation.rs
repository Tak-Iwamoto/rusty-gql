use super::gql_type::GraphQLType;

pub struct GraphQLMutation {
    name: String,
    description: Option<String>,
    return_type: GraphQLType,
}
