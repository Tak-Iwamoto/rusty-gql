use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, Mutex},
};

use graphql_parser::{
    query::{
        Definition, Document, Field, FragmentDefinition, Selection, SelectionSet,
        VariableDefinition,
    },
    schema::Directive,
};

use crate::{error::GqlError, types::schema::ArcSchema, GqlValueType, Schema, Variables};

#[derive(Debug)]
pub struct Operation<'a> {
    pub operation_type: OperationType,
    pub directives: Vec<Directive<'a, String>>,
    pub variable_definitions: Vec<VariableDefinition<'a, String>>,
    pub selection_set: SelectionSet<'a, String>,
    pub root_field: Field<'a, String>,
    pub fragment_definitions: HashMap<String, FragmentDefinition<'a, String>>,
    pub errors: Mutex<Vec<GqlError>>,
    pub variables: Variables,
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
struct OperationDefinition<'a> {
    operation_type: OperationType,
    directives: Vec<Directive<'a, String>>,
    variable_definitions: Vec<VariableDefinition<'a, String>>,
    selection_set: SelectionSet<'a, String>,
    root_field: Field<'a, String>,
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

pub fn get_operation_definitions<'a>(
    doc: &'a Document<'a, String>,
) -> Vec<&'a graphql_parser::query::Definition<'a, String>> {
    doc.definitions
        .iter()
        .filter(|def| matches!(def, Definition::Operation(_)))
        .collect::<Vec<_>>()
}

pub fn build_operation<'a>(
    doc: &'a Document<'a, String>,
    operation_name: Option<String>,
    variables: Variables,
    schema: &'a ArcSchema,
) -> Result<Operation<'a>, GqlError> {
    let mut fragment_definitions = HashMap::new();

    let mut operation_definitions: HashMap<String, OperationDefinition> = HashMap::new();
    let no_name_key = "no_operation_name";

    if operation_name.is_none() && get_operation_definitions(&doc).len() > 1 {
        return Err(GqlError::new(
            "Must provide operation name if multiple operation exist",
            None,
        ));
    };

    for definition in doc.clone().definitions {
        match definition {
            Definition::Operation(operation) => match operation {
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
                fragment_definitions.insert(name, fragment.to_owned());
            }
        }
    }

    match operation_name {
        Some(name) => {
            let target_def = operation_definitions.get(name.as_str());
            match target_def {
                Some(definition) => {
                    let definition = definition.clone();
                    Ok(Operation {
                        operation_type: definition.operation_type,
                        fragment_definitions,
                        directives: definition.directives,
                        variable_definitions: definition.variable_definitions,
                        selection_set: definition.selection_set,
                        root_field: definition.root_field,
                        errors: Default::default(),
                        variables,
                    })
                }
                None => Err(GqlError::new(
                    format!("{} is not contained in query", name),
                    None,
                )),
            }
        }
        None => match operation_definitions.get(&no_name_key.to_string()) {
            Some(definition) => {
                let definition = definition.clone();
                Ok(Operation {
                    operation_type: definition.operation_type,
                    fragment_definitions,
                    directives: definition.directives,
                    variable_definitions: definition.variable_definitions,
                    selection_set: definition.selection_set,
                    root_field: definition.root_field,
                    errors: Default::default(),
                    variables,
                })
            }
            None => match operation_definitions.values().next() {
                Some(definition) => {
                    let definition = definition.clone();
                    Ok(Operation {
                        operation_type: definition.operation_type,
                        fragment_definitions,
                        directives: definition.directives,
                        variable_definitions: definition.variable_definitions,
                        selection_set: definition.selection_set,
                        root_field: definition.root_field,
                        errors: Default::default(),
                        variables,
                    })
                }
                None => Err(GqlError::new("operation does not exist", None)),
            },
        },
    }
}

fn get_root_field<'a>(
    selection_set: &SelectionSet<'a, String>,
) -> Result<Field<'a, String>, GqlError> {
    let first_item = selection_set.items.first();
    if let Some(item) = first_item {
        if let Selection::Field(field) = item {
            return Ok(field.clone());
        }
    }
    Err(GqlError::new("A query must have root field", None))
}

fn get_operation_type<'a>(
    schema: &'a Schema,
    root_field: &Field<'a, String>,
) -> Result<OperationType, GqlError> {
    let root_fieldname = &root_field.name;

    if schema.queries.contains_key(root_fieldname) {
        return Ok(OperationType::Query);
    } else if root_fieldname == "__type"
        || root_fieldname == "__schema"
        || root_fieldname == "__typename"
    {
        return Ok(OperationType::Query);
    } else if schema.mutations.contains_key(root_fieldname) {
        return Ok(OperationType::Mutation);
    } else if schema.subscriptions.contains_key(root_fieldname) {
        return Ok(OperationType::Subscription);
    } else {
        Err(GqlError::new(
            format!("{} is not contained in schema", root_fieldname),
            None,
        ))
    }
}

pub fn get_root_field_ty<'a>(operation: &'a Operation<'a>, schema: &'a ArcSchema) -> GqlValueType {
    match operation.operation_type {
        OperationType::Query => schema
            .queries
            .get(&operation.root_field.name)
            .expect("must be defined in Query")
            .meta_type
            .clone(),
        OperationType::Mutation => schema
            .mutations
            .get(&operation.root_field.name)
            .expect("must be defined in Query")
            .meta_type
            .clone(),
        OperationType::Subscription => schema
            .subscriptions
            .get(&operation.root_field.name)
            .expect("must be defined in Query")
            .meta_type
            .clone(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::types::schema::{build_schema, ArcSchema};

    #[test]
    fn it_works() {
        let schema_doc = fs::read_to_string("tests/schemas/github.graphql").unwrap();
        let schema = ArcSchema::new(build_schema(&vec![schema_doc.as_str()]).unwrap());
        let query_doc = fs::read_to_string("tests/schemas/multiple_operation.graphql").unwrap();
        let parsed_query = graphql_parser::parse_query::<String>(&query_doc).unwrap();

        println!("{}", parsed_query.definitions.len());

        for def in parsed_query.definitions {
            println!("{:?}", def);
        }
        // let query = build_operation(query_doc.as_str(), &schema, None).unwrap();
        // println!("{:?}", &query.selection_set.items.len());
        // for item in query.selection_set.items {
        //     match item {
        //         graphql_parser::query::Selection::Field(field) => {
        //             println!("parent: {:?}", field);

        //             for it in field.selection_set.items {
        //                 println!("child: {:?}", it);
        //             }
        //         }
        //         graphql_parser::query::Selection::FragmentSpread(fragment_sp) => {
        //             println!("{}", fragment_sp.position);
        //         }
        //         graphql_parser::query::Selection::InlineFragment(inline_frg) => {
        //             println!("{}", inline_frg.position);
        //         }
        //     }
        // }
    }
}
