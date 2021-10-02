use std::collections::HashMap;

use crate::types::GraphQLOperationSchema;

use super::{gql_type::GraphQLType, GraphQLDirective};
use anyhow::Result;
use graphql_parser::{query::SelectionSet, schema::Text};

#[derive(Debug)]
pub struct GraphQLSchema {
    pub queries: HashMap<String, GraphQLOperationSchema>,
    pub mutations: HashMap<String, GraphQLOperationSchema>,
    pub subscriptions: HashMap<String, GraphQLOperationSchema>,
    pub directives: Vec<GraphQLDirective>,
    pub type_map: HashMap<String, GraphQLType>,
}

fn build_schema(schema_doc: &str) -> Result<GraphQLSchema> {
    let parsed_schema = graphql_parser::parse_schema::<&str>(schema_doc)?;
    let mut query_map = HashMap::new();
    let mut mutation_map = HashMap::new();
    let mut subscription_map = HashMap::new();
    let mut type_map = HashMap::new();

    for node in parsed_schema.definitions {
        match node {
            graphql_parser::schema::Definition::SchemaDefinition(schema) => {
                println!("{:?}", "schema");
                println!("{:?}", schema.query.unwrap());
            }
            graphql_parser::schema::Definition::TypeDefinition(type_def) => match type_def {
                graphql_parser::schema::TypeDefinition::Scalar(scalar) => {
                    println!("{:?}", scalar.name);
                }

                graphql_parser::schema::TypeDefinition::Object(obj) => match &*obj.name {
                    "Query" => {
                        for field in obj.fields {
                            let name = field.name.to_string();
                            let query = GraphQLOperationSchema::parse(field);
                            query_map.insert(name, query);
                        }
                    }
                    "Mutation" => {
                        for field in obj.fields {
                            let name = field.name.to_string();
                            let query = GraphQLOperationSchema::parse(field);
                            mutation_map.insert(name, query);
                        }
                    }
                    "Subscription" => {
                        for field in obj.fields {
                            let name = field.name.to_string();
                            let query = GraphQLOperationSchema::parse(field);
                            subscription_map.insert(name, query);
                        }
                    }
                    _ => {
                        println!("{:?}", obj.name);
                        // for field in obj.fields {

                        // }
                    }
                },
                graphql_parser::schema::TypeDefinition::Interface(interface) => {
                    println!("{:?}", "interface");
                    println!("{:?}", interface.name);
                }
                graphql_parser::schema::TypeDefinition::Union(union) => {
                    println!("{:?}", union.name);
                }
                graphql_parser::schema::TypeDefinition::Enum(enu) => {
                    println!("{:?}", enu.name);
                }
                graphql_parser::schema::TypeDefinition::InputObject(input) => {
                    println!("{:?}", "input");
                    println!("{:?}", input.name);
                }
            },
            graphql_parser::schema::Definition::TypeExtension(type_ext) => {
                println!("{:?}", "type_ext");
                println!("{:?}", type_ext);
            }
            graphql_parser::schema::Definition::DirectiveDefinition(directive) => {
                println!("{:?}", "directive");
                println!("{:?}", directive);
            }
        }
    }
    Ok(GraphQLSchema {
        queries: query_map,
        mutations: mutation_map,
        subscriptions: subscription_map,
        directives: vec![],
        type_map,
    })
}

fn parse_selection_set<'a, T: Text<'a>>(selection_set: SelectionSet<'a, T>) {
    for item in selection_set.items {
        match item {
            graphql_parser::query::Selection::Field(field) => {
                println!("{:?}", field.name);
                parse_selection_set(field.selection_set);
            }
            graphql_parser::query::Selection::FragmentSpread(_) => todo!(),
            graphql_parser::query::Selection::InlineFragment(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::build_schema;

    #[test]
    fn it_works() {
        let contents = fs::read_to_string("src/tests/github.graphql");
        let v = contents.unwrap();
        let schema = build_schema(v.as_str());
    }
}
