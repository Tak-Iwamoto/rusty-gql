use super::gql_type::GraphQLType;

pub struct GraphQLQuery {
    name: String,
    description: Option<String>,
    return_type: GraphQLType,
}
