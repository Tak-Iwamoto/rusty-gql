use std::collections::HashMap;

use super::{
    field::GraphQLField, interface::GraphQLInterface, query::GraphQLQuery, GraphQLDirective,
};
use anyhow::Result;

pub struct GraphQLObject {
    pub name: String,
    pub description: Option<String>,
    pub fields: HashMap<String, GraphQLField>,
    pub interfaces: Vec<GraphQLInterface>,
}

pub struct GraphQLSchema {
    // 一旦適当
    extensions: Vec<String>,
    pub query: HashMap<String, GraphQLQuery>,
    pub mutation: GraphQLObject,
    pub subscription: GraphQLObject,
    pub directives: Vec<GraphQLDirective>,
}

fn build_schema(schema_doc: &str) -> Result<()> {
    let parsed_schema = graphql_parser::parse_schema::<&str>(schema_doc)?;
    for node in parsed_schema.definitions {
        match node {
            graphql_parser::schema::Definition::SchemaDefinition(schema) => {
                println!("{:?}", "query");
                println!("{:?}", schema.query);
            }
            graphql_parser::schema::Definition::TypeDefinition(type_def) => match type_def {
                graphql_parser::schema::TypeDefinition::Scalar(scalar) => {
                    println!("{:?}", scalar.name);
                }
                graphql_parser::schema::TypeDefinition::Object(obj) => {
                    println!("{:?}", "obj");
                    println!("{:?}", obj.name);
                }
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::build_schema;

    #[test]
    fn it_works() {
        let contents = fs::read_to_string("src/tests/github.graphql");
        let v = contents.unwrap();
        build_schema(v.as_str());
    }
}
