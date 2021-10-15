use std::{collections::HashSet};

use crate::{
    operation::GraphQLOperation, resolver::Resolver, types::GraphQLFragmentDefinition,
    GraphQLError, GraphQLSchema,
};

pub struct ExecutorContext<'a> {
    schema: GraphQLSchema,
    operation: GraphQLOperation<'a>,
}

// impl<'a> ExecutorContext<'a> {
//     pub fn collect_fields(&self) {
//         for  def in self.operation.definitions {
//             match def {
//                 graphql_parser::query::Definition::Operation(operation_def) => {

//                 },
//                 graphql_parser::query::Definition::Fragment(_) => todo!(),
//             }

//         }

//     }

// }

pub struct Executor {
    schema: GraphQLSchema,
    fragments: HashSet<String, GraphQLFragmentDefinition>,
    // 一旦valueをstringにする
    variables: HashSet<String, String>,
    field_resolver: Box<dyn Resolver>,
    type_resolver: Box<dyn Resolver>,
    errors: Vec<GraphQLError>,
}
