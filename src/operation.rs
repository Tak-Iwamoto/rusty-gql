use std::collections::{BTreeMap, HashMap};

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
    pub definition: GraphQLOperationDefinition<'a>,
    pub fragments: BTreeMap<String, FragmentDefinition<'a, &'a str>>,
    // pub variables:
    // pub errors
}

#[derive(Clone)]
pub struct GraphQLOperationDefinition<'a> {
    pub name: Option<String>,
    pub operaton_type: GraphQLOperationType,
    pub directives: Vec<Directive<'a, &'a str>>,
    pub variable_definitions: Vec<VariableDefinition<'a, &'a str>>,
    pub selection_set: SelectionSet<'a, &'a str>,
}

// operation_nameがある場合はここでひとつだけ返すで良さそう
pub fn build_operation<'a>(
    query_doc: &'a str,
    operation_name: Option<&str>,
) -> Result<GraphQLOperation<'a>, String> {
    let parsed_query = graphql_parser::parse_query::<&str>(query_doc).unwrap();

    let mut fragments = BTreeMap::new();

    let mut operation_definitions: HashMap<&str, GraphQLOperationDefinition> = HashMap::new();
    let no_name_key = "no_name";

    if operation_name.is_none() && parsed_query.definitions.len() > 1 {
        return Err(String::from(
            "Must provide operation name if multiple operation exist",
        ));
    };

    for definition in parsed_query.definitions {
        match definition {
            graphql_parser::query::Definition::Operation(operation) => match operation {
                graphql_parser::query::OperationDefinition::SelectionSet(selection_set) => {
                    if operation_name.is_none() {
                        operation_definitions.insert(
                            no_name_key,
                            GraphQLOperationDefinition {
                                name: None,
                                operaton_type: GraphQLOperationType::Query,
                                selection_set,
                                directives: vec![],
                                variable_definitions: vec![],
                            },
                        );
                    }
                }
                graphql_parser::query::OperationDefinition::Query(query) => {
                    let query_name = query.name.unwrap_or_else(|| no_name_key);
                    operation_definitions.insert(
                        query_name,
                        GraphQLOperationDefinition {
                            name: query.name.map(|s| s.to_string()),
                            operaton_type: GraphQLOperationType::Query,
                            selection_set: query.selection_set,
                            directives: query.directives,
                            variable_definitions: query.variable_definitions,
                        },
                    );
                }
                graphql_parser::query::OperationDefinition::Mutation(mutation) => {
                    let mutation_name = mutation.name.unwrap_or_else(|| no_name_key);
                    operation_definitions.insert(
                        mutation_name,
                        GraphQLOperationDefinition {
                            name: mutation.name.map(|s| s.to_string()),
                            operaton_type: GraphQLOperationType::Mutation,
                            selection_set: mutation.selection_set,
                            directives: mutation.directives,
                            variable_definitions: mutation.variable_definitions,
                        },
                    );
                }
                graphql_parser::query::OperationDefinition::Subscription(subscription) => {
                    let subscription_name = subscription.name.unwrap_or_else(|| no_name_key);
                    operation_definitions.insert(
                        subscription_name,
                        GraphQLOperationDefinition {
                            name: subscription.name.map(|s| s.to_string()),
                            operaton_type: GraphQLOperationType::Subscription,
                            selection_set: subscription.selection_set,
                            directives: subscription.directives,
                            variable_definitions: subscription.variable_definitions,
                        },
                    );
                }
            },
            graphql_parser::query::Definition::Fragment(fragment) => {
                let name = fragment.name.to_string();
                fragments.insert(name, fragment.to_owned());
            }
        }
    }

    match operation_name {
        Some(name) => {
            let target_def = operation_definitions.get(name);
            match target_def {
                Some(definition) => Ok(GraphQLOperation {
                    definition: definition.clone(),
                    fragments,
                }),
                None => Err(format!("{} is not query name", name)),
            }
        }
        None => match operation_definitions.get(no_name_key) {
            Some(definition) => Ok(GraphQLOperation {
                definition: definition.clone(),
                fragments,
            }),
            None => Err(String::from("operation does not exist")),
        },
    }
}
