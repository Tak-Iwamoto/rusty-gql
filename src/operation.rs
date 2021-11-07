use std::collections::{BTreeMap, HashMap};

use graphql_parser::{
    query::{Field, FragmentDefinition, SelectionSet, VariableDefinition},
    schema::Directive,
};

#[derive(Debug)]
pub struct GraphQLOperation<'a> {
    pub definition: GraphQLOperationDefinition<'a>,
    pub fragments: BTreeMap<String, FragmentDefinition<'a, &'a str>>,
    // pub variables:
    // pub errors
}

#[derive(Clone, Debug)]
pub struct GraphQLOperationDefinition<'a> {
    pub directives: Vec<Directive<'a, &'a str>>,
    pub variable_definitions: Vec<VariableDefinition<'a, &'a str>>,
    pub selection_set: SelectionSet<'a, &'a str>,
    pub root_field: Field<'a, &'a str>,
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
                        let root_field = get_root_field(&selection_set)?;
                        operation_definitions.insert(
                            no_name_key,
                            GraphQLOperationDefinition {
                                selection_set,
                                root_field,
                                directives: vec![],
                                variable_definitions: vec![],
                            },
                        );
                    }
                }
                graphql_parser::query::OperationDefinition::Query(query) => {
                    let query_name = query.name.unwrap_or_else(|| no_name_key);
                    let root_field = get_root_field(&query.selection_set)?;
                    operation_definitions.insert(
                        query_name,
                        GraphQLOperationDefinition {
                            selection_set: query.selection_set,
                            root_field,
                            directives: query.directives,
                            variable_definitions: query.variable_definitions,
                        },
                    );
                }
                graphql_parser::query::OperationDefinition::Mutation(mutation) => {
                    let mutation_name = mutation.name.unwrap_or_else(|| no_name_key);
                    let root_field = get_root_field(&mutation.selection_set)?;
                    operation_definitions.insert(
                        mutation_name,
                        GraphQLOperationDefinition {
                            selection_set: mutation.selection_set,
                            root_field,
                            directives: mutation.directives,
                            variable_definitions: mutation.variable_definitions,
                        },
                    );
                }
                graphql_parser::query::OperationDefinition::Subscription(subscription) => {
                    let subscription_name = subscription.name.unwrap_or_else(|| no_name_key);
                    let root_field = get_root_field(&subscription.selection_set)?;
                    operation_definitions.insert(
                        subscription_name,
                        GraphQLOperationDefinition {
                            selection_set: subscription.selection_set,
                            root_field,
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

fn get_root_field<'a>(
    selection_set: &SelectionSet<'a, &'a str>,
) -> Result<Field<'a, &'a str>, String> {
    let first_item = selection_set.items.first();
    match first_item {
        Some(item) => match item {
            graphql_parser::query::Selection::Field(field) => Ok(field.clone()),
            graphql_parser::query::Selection::FragmentSpread(_) => unreachable!(),
            graphql_parser::query::Selection::InlineFragment(_) => unreachable!(),
        },
        None => Err(String::from("Must have selection item")),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::build_operation;

    #[test]
    fn it_works() {
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let query = build_operation(query_doc.as_str(), None).unwrap();

        println!("{:?}", query.definition.root_field);
    }
}
