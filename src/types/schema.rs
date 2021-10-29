use std::collections::BTreeMap;

use super::GraphQLType;
use anyhow::Result;
use graphql_parser::schema::{DirectiveDefinition, Field};

#[derive(Debug)]
pub struct GraphQLSchema<'a> {
    pub queries: BTreeMap<String, Field<'a, &'a str>>,
    pub mutations: BTreeMap<String, Field<'a, &'a str>>,
    pub subscriptions: BTreeMap<String, Field<'a, &'a str>>,
    pub directives: BTreeMap<String, DirectiveDefinition<'a, &'a str>>,
    pub type_map: BTreeMap<String, GraphQLType<'a>>,
}

pub fn build_schema(schema_doc: &str) -> Result<GraphQLSchema> {
    let parsed_schema = graphql_parser::parse_schema::<&str>(schema_doc)?;
    let mut query_map = BTreeMap::new();
    let mut mutation_map = BTreeMap::new();
    let mut subscription_map = BTreeMap::new();
    let mut type_map = BTreeMap::new();
    let mut directive_map = BTreeMap::new();

    for node in parsed_schema.definitions {
        match node {
            // TODO:
            graphql_parser::schema::Definition::SchemaDefinition(schema) => {}
            graphql_parser::schema::Definition::TypeDefinition(type_def) => match type_def {
                graphql_parser::schema::TypeDefinition::Scalar(scalar) => {
                    let name = scalar.name.to_string();
                    type_map.insert(name, GraphQLType::Scalar(scalar));
                }

                graphql_parser::schema::TypeDefinition::Object(obj) => match &*obj.name {
                    "Query" => {
                        for field in obj.fields {
                            let name = field.name.to_string();
                            query_map.insert(name, field);
                        }
                    }
                    "Mutation" => {
                        for field in obj.fields {
                            let name = field.name.to_string();
                            mutation_map.insert(name, field);
                        }
                    }
                    "Subscription" => {
                        for field in obj.fields {
                            let name = field.name.to_string();
                            subscription_map.insert(name, field);
                        }
                    }
                    _ => {
                        let name = obj.name.to_string();
                        type_map.insert(name, GraphQLType::Object(obj));
                    }
                },
                graphql_parser::schema::TypeDefinition::Interface(interface) => {
                    let name = interface.name.to_string();
                    type_map.insert(name, GraphQLType::Interface(interface));
                }
                graphql_parser::schema::TypeDefinition::Union(uni) => {
                    let name = uni.name.to_string();
                    type_map.insert(name, GraphQLType::Union(uni));
                }
                graphql_parser::schema::TypeDefinition::Enum(enu) => {
                    let name = enu.name.to_string();
                    type_map.insert(name, GraphQLType::Enum(enu));
                }
                graphql_parser::schema::TypeDefinition::InputObject(input) => {
                    let name = input.name.to_string();
                    type_map.insert(name, GraphQLType::Input(input));
                }
            },
            // TODO:
            graphql_parser::schema::Definition::TypeExtension(type_ext) => {}
            graphql_parser::schema::Definition::DirectiveDefinition(directive) => {
                let name = directive.name.to_string();
                directive_map.insert(name, directive);
            }
        }
    }
    Ok(GraphQLSchema {
        queries: query_map,
        mutations: mutation_map,
        subscriptions: subscription_map,
        directives: directive_map,
        type_map,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::build_schema;

    #[test]
    fn it_works() {
        let contents = fs::read_to_string("src/tests/github.graphql");
        let v = contents.unwrap();
        let schema = build_schema(v.as_str()).unwrap();
        let query = schema.queries.get("codeOfConduct").unwrap();
    }
}
