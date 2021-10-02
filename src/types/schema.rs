use std::collections::HashMap;

use crate::types::{argument::GraphQLArgument, gql_type::GraphQLType};

use super::{
    mutation::GraphQLMutation, query::GraphQLQuery, subscription::GraphQLSubscription,
    GraphQLDirective,
};
use anyhow::Result;
use graphql_parser::{query::SelectionSet, schema::Text};

pub struct GraphQLSchema {
    pub queries: HashMap<String, GraphQLQuery>,
    pub mutations: HashMap<String, GraphQLMutation>,
    pub subscriptions: HashMap<String, GraphQLSubscription>,
    pub directives: Vec<GraphQLDirective>,
}

fn build_schema(schema_doc: &str) -> Result<()> {
    let parsed_schema = graphql_parser::parse_schema::<&str>(schema_doc)?;

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
                // Query, Mutationもobject扱いになっている
                graphql_parser::schema::TypeDefinition::Object(obj) => {
                    match &*obj.name {
                        "Query" => {
                            for field in obj.fields {
                                println!("{:?}", field.name);
                                let name = field.name.to_string();
                                let args: Vec<GraphQLArgument> = field
                                    .arguments
                                    .into_iter()
                                    .map(|field| GraphQLArgument::parse(field))
                                    .collect();
                                println!("{:?}", field.directives); // field_typeが戻り値
                                let query = GraphQLQuery {
                                    name,
                                    args,
                                    description: None,
                                    directives: vec![],
                                    return_type: GraphQLType::Null,
                                };
                                println!("{:?}", query);
                                // match field.field_type {
                                //     graphql_parser::schema::Type::NamedType(_) => todo!(),
                                //     graphql_parser::schema::Type::ListType(_) => todo!(),
                                //     graphql_parser::schema::Type::NonNullType(_) => todo!(),
                                // }
                            }
                        }
                        "Mutation" => {
                            println!("{:?}", "mutation");
                        }
                        "Subscription" => {
                            println!("{:?}", "subscription");
                        }
                        _ => {}
                    }
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
        let contents = fs::read_to_string("src/tests/github_schema_query.graphql");
        let v = contents.unwrap();
        build_schema(v.as_str());
    }
}
