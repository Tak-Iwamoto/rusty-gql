use std::{
    collections::{BTreeMap, HashMap},
    ops::Deref,
    sync::Arc,
};

use graphql_parser::{
    query::{Field, FragmentDefinition, SelectionSet, VariableDefinition},
    schema::Directive,
};

use crate::{types::schema::ArcSchema, Schema};

#[derive(Debug, Clone)]
pub struct Operation<'a> {
    pub definition: OperationDefinition<'a>,
    pub fragments: BTreeMap<String, FragmentDefinition<'a, String>>,
    // pub variables:
    // pub errors
}

#[derive(Debug)]
pub struct ArcOperation<'a>(Arc<Operation<'a>>);

impl<'a> ArcOperation<'a> {
    pub fn new(operation: Operation<'a>) -> ArcOperation<'a> {
        ArcOperation(Arc::new(operation))
    }
}

impl<'a> Deref for ArcOperation<'a> {
    type Target = Operation<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct OperationDefinition<'a> {
    pub operation_type: OperationType,
    pub directives: Vec<Directive<'a, String>>,
    pub variable_definitions: Vec<VariableDefinition<'a, String>>,
    pub selection_set: SelectionSet<'a, String>,
    pub root_field: Field<'a, String>,
}

#[derive(Clone, Debug)]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

impl ToString for OperationType {
    fn to_string(&self) -> String {
        match self {
            OperationType::Query => String::from("Query"),
            OperationType::Mutation => String::from("Mutation"),
            OperationType::Subscription => String::from("Subscription"),
        }
    }
}

pub fn build_operation<'a>(
    query_doc: &'a str,
    schema: &'a ArcSchema,
    operation_name: Option<String>,
) -> Result<Operation<'a>, String> {
    let parsed_query = match graphql_parser::parse_query::<String>(query_doc) {
        Ok(parsed) => parsed,
        Err(_) => return Err(String::from("failed to parse query")),
    };

    let mut fragments = BTreeMap::new();

    let mut operation_definitions: HashMap<String, OperationDefinition> = HashMap::new();
    let no_name_key = "no_operation_name";

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
                        let operation_type = get_operation_type(&schema, &root_field)?;
                        operation_definitions.insert(
                            no_name_key.to_string(),
                            OperationDefinition {
                                operation_type,
                                selection_set,
                                root_field,
                                directives: vec![],
                                variable_definitions: vec![],
                            },
                        );
                    }
                }
                graphql_parser::query::OperationDefinition::Query(query) => {
                    let query_name = query.name.unwrap_or_else(|| no_name_key.to_string());
                    let root_field = get_root_field(&query.selection_set)?;
                    let operation_type = get_operation_type(&schema, &root_field)?;
                    operation_definitions.insert(
                        query_name,
                        OperationDefinition {
                            operation_type,
                            selection_set: query.selection_set,
                            root_field,
                            directives: query.directives,
                            variable_definitions: query.variable_definitions,
                        },
                    );
                }
                graphql_parser::query::OperationDefinition::Mutation(mutation) => {
                    let mutation_name = mutation.name.unwrap_or_else(|| no_name_key.to_string());
                    let root_field = get_root_field(&mutation.selection_set)?;
                    let operation_type = get_operation_type(&schema, &root_field)?;
                    operation_definitions.insert(
                        mutation_name,
                        OperationDefinition {
                            operation_type,
                            selection_set: mutation.selection_set,
                            root_field,
                            directives: mutation.directives,
                            variable_definitions: mutation.variable_definitions,
                        },
                    );
                }
                graphql_parser::query::OperationDefinition::Subscription(subscription) => {
                    let subscription_name =
                        subscription.name.unwrap_or_else(|| no_name_key.to_string());
                    let root_field = get_root_field(&subscription.selection_set)?;
                    let operation_type = get_operation_type(&schema, &root_field)?;
                    operation_definitions.insert(
                        subscription_name,
                        OperationDefinition {
                            operation_type,
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
            let target_def = operation_definitions.get(name.as_str());
            match target_def {
                Some(definition) => Ok(Operation {
                    definition: definition.clone(),
                    fragments,
                }),
                None => Err(format!("{} is not contained in query", name)),
            }
        }
        None => match operation_definitions.get(&no_name_key.to_string()) {
            Some(definition) => Ok(Operation {
                definition: definition.clone(),
                fragments,
            }),
            None => match operation_definitions.values().next() {
                Some(definition) => Ok(Operation {
                    definition: definition.clone(),
                    fragments,
                }),
                None => Err(String::from("operation does not exist")),
            },
        },
    }
}

fn get_root_field<'a>(
    selection_set: &SelectionSet<'a, String>,
) -> Result<Field<'a, String>, String> {
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
fn get_operation_type<'a>(
    schema: &'a Schema,
    root_field: &Field<'a, String>,
) -> Result<OperationType, String> {
    let root_fieldname = &root_field.name;

    if schema.queries.contains_key(root_fieldname) {
        return Ok(OperationType::Query);
    } else if schema.mutations.contains_key(root_fieldname) {
        return Ok(OperationType::Mutation);
    } else if schema.subscriptions.contains_key(root_fieldname) {
        return Ok(OperationType::Subscription);
    } else {
        Err(format!("{} is not contained in schema", root_fieldname))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::types::schema::{build_schema, ArcSchema};

    use super::build_operation;

    #[test]
    fn it_works() {
        let schema_doc = fs::read_to_string("src/tests/github.graphql").unwrap();
        let schema = ArcSchema::new(build_schema(schema_doc.as_str()).unwrap());
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let query = build_operation(query_doc.as_str(), &schema, None).unwrap();

        println!("{:?}", query.definition.root_field);
        println!(
            "{:?}",
            schema
                .queries
                .get(&query.definition.root_field.name.to_string())
                .unwrap()
        );
        println!("{:?}", query.definition);
    }
}
