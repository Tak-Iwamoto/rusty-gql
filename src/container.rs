use std::collections::HashMap;

use futures::future::BoxFuture;

use crate::{graphql_value::GraphQLValue, GraphQLResponse};

// Container holds the information of the resolver
type GraphQLFuture<'a> = BoxFuture<'a, GraphQLResponse<GraphQLValue>>;

pub struct Container<'a> {
    // key is parent_type and target field
    // value is resolve fn
    resolvers: HashMap<(String, String), GraphQLFuture<'a>>,
}
