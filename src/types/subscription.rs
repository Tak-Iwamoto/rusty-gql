use super::gql_type::GraphQLType;

pub struct GraphQLSubscription {
    name: String,
    description: Option<String>,
    return_type: GraphQLType,
}
