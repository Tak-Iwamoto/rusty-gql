use std::collections::HashSet;

use crate::{GraphQLError, GraphQLSchema, resolver::Resolver, types::GraphQLFragment};

pub struct Executor {
    schema: GraphQLSchema,
    fragments: HashSet<String, GraphQLFragment>,
    // 一旦valueをstringにする
    variables: HashSet<String, String>,
    field_resolver: Box<dyn Resolver>,
    type_resolver: Box<dyn Resolver>,
    errors: Vec<GraphQLError>
}

impl Executor {
    pub fn execute(&self) {

    }


    fn execute_operation(&self) {

    }
}
