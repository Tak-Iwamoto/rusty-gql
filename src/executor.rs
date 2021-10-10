use std::collections::{HashMap, HashSet};

use crate::{resolver::Resolver, types::GraphQLFragmentDefinition, GraphQLError, GraphQLSchema};
use graphql_parser::query::{Field, FragmentDefinition, Selection, SelectionSet};

pub struct Executor {
    schema: GraphQLSchema,
    fragments: HashSet<String, GraphQLFragmentDefinition>,
    // 一旦valueをstringにする
    variables: HashSet<String, String>,
    field_resolver: Box<dyn Resolver>,
    type_resolver: Box<dyn Resolver>,
    errors: Vec<GraphQLError>,
}

impl Executor {
    pub fn execute(&self) {}

    fn execute_operation(&self) {}
}
