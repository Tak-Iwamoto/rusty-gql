use std::collections::HashMap;

use super::{
    directive::GraphQLDirectiveDefinition, GraphQLEnum, GraphQLField, GraphQLInput,
    GraphQLInterface, GraphQLObjectType, GraphQLScalar, GraphQLType, GraphQLUnion,
};
use anyhow::Result;

#[derive(Debug)]
pub struct GraphQLSchema {
    pub queries: HashMap<String, GraphQLField>,
    pub mutations: HashMap<String, GraphQLField>,
    pub subscriptions: HashMap<String, GraphQLField>,
    pub directives: HashMap<String, GraphQLDirectiveDefinition>,
    pub type_map: HashMap<String, GraphQLType>,
}

pub fn build_schema(schema_doc: &str) -> Result<GraphQLSchema> {
    let parsed_schema = graphql_parser::parse_schema::<&str>(schema_doc)?;
    let mut query_map = HashMap::new();
    let mut mutation_map = HashMap::new();
    let mut subscription_map = HashMap::new();
    let mut type_map = HashMap::new();
    let mut directive_map = HashMap::new();

    for node in parsed_schema.definitions {
        match node {
            // TODO:
            graphql_parser::schema::Definition::SchemaDefinition(schema) => {}
            graphql_parser::schema::Definition::TypeDefinition(type_def) => match type_def {
                graphql_parser::schema::TypeDefinition::Scalar(scalar) => {
                    let name = scalar.name.to_string();
                    let gql_scalar = GraphQLScalar::parse(scalar);
                    type_map.insert(name, GraphQLType::GraphQLScalar(gql_scalar));
                }

                graphql_parser::schema::TypeDefinition::Object(obj) => match &*obj.name {
                    "Query" => {
                        for field in obj.fields {
                            let name = field.name.to_string();
                            let query = GraphQLField::parse(field);
                            query_map.insert(name, query);
                        }
                    }
                    "Mutation" => {
                        for field in obj.fields {
                            let name = field.name.to_string();
                            let query = GraphQLField::parse(field);
                            mutation_map.insert(name, query);
                        }
                    }
                    "Subscription" => {
                        for field in obj.fields {
                            let name = field.name.to_string();
                            let query = GraphQLField::parse(field);
                            subscription_map.insert(name, query);
                        }
                    }
                    _ => {
                        let name = obj.name.to_string();
                        let gql_object = GraphQLObjectType::parse(obj);
                        type_map.insert(name, GraphQLType::GraphQLObject(gql_object));
                    }
                },
                graphql_parser::schema::TypeDefinition::Interface(interface) => {
                    let name = interface.name.to_string();
                    let gql_interface = GraphQLInterface::parse(interface);
                    type_map.insert(name, GraphQLType::GraphQLInterface(gql_interface));
                }
                graphql_parser::schema::TypeDefinition::Union(uni) => {
                    let name = uni.name.to_string();
                    let gql_union = GraphQLUnion::parse(uni);
                    type_map.insert(name, GraphQLType::GraphQLUnion(gql_union));
                }
                graphql_parser::schema::TypeDefinition::Enum(enu) => {
                    let name = enu.name.to_string();
                    let gql_enum = GraphQLEnum::parse(enu);
                    type_map.insert(name, GraphQLType::GraphQLEnum(gql_enum));
                }
                graphql_parser::schema::TypeDefinition::InputObject(input) => {
                    let name = input.name.to_string();
                    let gql_input = GraphQLInput::parse(input);
                    type_map.insert(name, GraphQLType::GraphQLInput(gql_input));
                }
            },
            // TODO:
            graphql_parser::schema::Definition::TypeExtension(type_ext) => {}
            graphql_parser::schema::Definition::DirectiveDefinition(directive) => {
                let name = directive.name.to_string();
                let gql_directive = GraphQLDirectiveDefinition::parse(directive);
                directive_map.insert(name, gql_directive);
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
