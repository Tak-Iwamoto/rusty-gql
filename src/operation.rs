use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use graphql_parser::{
    query::{FragmentDefinition, SelectionSet, VariableDefinition},
    schema::Directive,
};

#[derive(Clone)]
pub enum GraphQLOperationType {
    Query,
    Mutation,
    Subscription,
}

pub struct GraphQLOperation<'a> {
    pub operation_request: OperationRequest<'a>,
    pub operation_name: Option<String>,
    pub fragments: BTreeMap<String, FragmentDefinition<'a, &'a str>>,
    // pub variables:
    // pub errors
}

#[derive(Clone)]
pub struct GQLOperationDefinition<'a> {
    pub name: Option<String>,
    pub operaton_type: GraphQLOperationType,
    pub directives: Vec<Directive<'a, &'a str>>,
    pub variable_definitions: Vec<VariableDefinition<'a, &'a str>>,
    pub selection_set: SelectionSet<'a, &'a str>,
}

#[derive(Clone)]
pub enum OperationRequest<'a> {
    Single(GQLOperationDefinition<'a>),
    Multi(Vec<GQLOperationDefinition<'a>>),
}

pub fn build_operation(
    query_doc: &str,
    operation_name: Option<String>,
) -> Result<GraphQLOperation> {
    let parsed_query = graphql_parser::parse_query::<&str>(query_doc)?;

    let mut fragments = BTreeMap::new();

    let mut operations: Vec<GQLOperationDefinition> = Vec::new();
    for definition in parsed_query.definitions {
        match definition {
            graphql_parser::query::Definition::Operation(operation) => match operation {
                graphql_parser::query::OperationDefinition::SelectionSet(selection_set) => {
                    operations.push(GQLOperationDefinition {
                        name: None,
                        operaton_type: GraphQLOperationType::Query,
                        selection_set,
                        directives: vec![],
                        variable_definitions: vec![],
                    })
                }
                graphql_parser::query::OperationDefinition::Query(query) => {
                    operations.push(GQLOperationDefinition {
                        name: query.name.map(|s| s.to_string()),
                        operaton_type: GraphQLOperationType::Query,
                        selection_set: query.selection_set,
                        directives: query.directives,
                        variable_definitions: query.variable_definitions,
                    })
                }
                graphql_parser::query::OperationDefinition::Mutation(mutation) => {
                    operations.push(GQLOperationDefinition {
                        name: mutation.name.map(|s| s.to_string()),
                        operaton_type: GraphQLOperationType::Query,
                        selection_set: mutation.selection_set,
                        directives: mutation.directives,
                        variable_definitions: mutation.variable_definitions,
                    })
                }
                graphql_parser::query::OperationDefinition::Subscription(subscription) => {
                    operations.push(GQLOperationDefinition {
                        name: subscription.name.map(|s| s.to_string()),
                        operaton_type: GraphQLOperationType::Query,
                        selection_set: subscription.selection_set,
                        directives: subscription.directives,
                        variable_definitions: subscription.variable_definitions,
                    })
                }
            },
            graphql_parser::query::Definition::Fragment(fragment) => {
                let name = fragment.name.to_string();
                fragments.insert(name, fragment.to_owned());
            }
        }
    }

    let operation_request: Result<OperationRequest> = match operations.len() {
        0 => Err(anyhow!("operation does not exist")),
        1 => Ok(OperationRequest::Single(operations.get(0).unwrap().clone())),
        _ => Ok(OperationRequest::Multi(operations)),
    };

    Ok(GraphQLOperation {
        operation_request: operation_request?,
        operation_name,
        fragments,
    })
}
