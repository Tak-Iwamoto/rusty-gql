use std::collections::HashSet;

use crate::{
    operation::GraphQLOperation, resolver::Resolver, types::GraphQLFragmentDefinition,
    GraphQLError, GraphQLSchema,
};

pub struct ExecutorContext<'a> {
    schema: GraphQLSchema,
    operation: GraphQLOperation<'a>,
}

pub struct Executor {
    schema: GraphQLSchema,
    fragments: HashSet<String, GraphQLFragmentDefinition>,
    // 一旦valueをstringにする
    variables: HashSet<String, String>,
    field_resolver: Box<dyn Resolver>,
    type_resolver: Box<dyn Resolver>,
    errors: Vec<GraphQLError>,
}
