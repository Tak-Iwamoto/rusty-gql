use std::{collections::BTreeMap, ops::Deref, sync::Arc};

use super::GraphQLType;
use graphql_parser::schema::{
    DirectiveDefinition, EnumType, Field, InputObjectType, InterfaceType, ObjectType, ScalarType,
    UnionType,
};

#[derive(Debug, Clone)]
pub struct Schema<'a> {
    pub queries: BTreeMap<String, Field<'a, String>>,
    pub mutations: BTreeMap<String, Field<'a, String>>,
    pub subscriptions: BTreeMap<String, Field<'a, String>>,
    pub directives: BTreeMap<String, DirectiveDefinition<'a, String>>,
    pub type_map: BTreeMap<String, GraphQLType<'a>>,
}

pub struct ArcSchema<'a>(Arc<Schema<'a>>);

impl<'a> ArcSchema<'a> {
    pub fn new(schema: Schema<'a>) -> Self {
        ArcSchema(Arc::new(schema))
    }
}

impl<'a> Deref for ArcSchema<'a> {
    type Target = Schema<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn build_schema(schema_doc: &str) -> Result<Schema, String> {
    let parsed_schema =
        graphql_parser::parse_schema::<String>(schema_doc).expect("failed to parse graphql schema");
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
                    type_map.insert(scalar.name.to_string(), GraphQLType::Scalar(scalar));
                }

                graphql_parser::schema::TypeDefinition::Object(obj) => match &*obj.name {
                    "Query" => {
                        for f in obj.fields {
                            query_map.insert(f.name.to_string(), f);
                        }
                    }
                    "Mutation" => {
                        for f in obj.fields {
                            mutation_map.insert(f.name.to_string(), f);
                        }
                    }
                    "Subscription" => {
                        for f in obj.fields {
                            subscription_map.insert(f.name.to_string(), f);
                        }
                    }
                    _ => {
                        type_map.insert(obj.name.to_string(), GraphQLType::Object(obj));
                    }
                },
                graphql_parser::schema::TypeDefinition::Interface(interface) => {
                    type_map.insert(
                        interface.name.to_string(),
                        GraphQLType::Interface(interface),
                    );
                }
                graphql_parser::schema::TypeDefinition::Union(uni) => {
                    type_map.insert(uni.name.to_string(), GraphQLType::Union(uni));
                }
                graphql_parser::schema::TypeDefinition::Enum(enu) => {
                    type_map.insert(enu.name.to_string(), GraphQLType::Enum(enu));
                }
                graphql_parser::schema::TypeDefinition::InputObject(input) => {
                    type_map.insert(input.name.to_string(), GraphQLType::Input(input));
                }
            },
            graphql_parser::schema::Definition::TypeExtension(type_ext) => match type_ext {
                graphql_parser::schema::TypeExtension::Scalar(scalar_ext) => {
                    let original_name = scalar_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_scalar) => {
                            if let GraphQLType::Scalar(original) = original_scalar {
                                let mut extended_directives = original.directives.clone();
                                extended_directives.append(&mut scalar_ext.directives.clone());

                                let extended_scalar = ScalarType {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                };
                                type_map
                                    .insert(original_name, GraphQLType::Scalar(extended_scalar));
                            }
                        }
                        None => return Err(String::from("The scalar to extend is not found")),
                    }
                }
                graphql_parser::schema::TypeExtension::Object(obj_ext) => {
                    let original_name = obj_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_obj) => {
                            if let GraphQLType::Object(original) = original_obj {
                                let mut extended_directives = original.directives.clone();
                                extended_directives.append(&mut obj_ext.directives.clone());

                                let mut extended_fields = original.fields.clone();
                                extended_fields.append(&mut obj_ext.fields.clone());

                                let mut extended_impl_interfaces =
                                    original.implements_interfaces.clone();
                                extended_impl_interfaces
                                    .append(&mut obj_ext.implements_interfaces.clone());

                                let extended_obj = ObjectType {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    fields: extended_fields,
                                    implements_interfaces: extended_impl_interfaces,
                                };
                                type_map.insert(
                                    original_name.to_string(),
                                    GraphQLType::Object(extended_obj),
                                );
                            }
                        }
                        None => return Err(String::from("The interface to extend is not found")),
                    }
                }
                graphql_parser::schema::TypeExtension::Interface(inter_ext) => {
                    let original_name = inter_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_interface) => {
                            if let GraphQLType::Interface(original) = original_interface {
                                let mut extended_directives = original.directives.clone();
                                extended_directives.append(&mut inter_ext.directives.clone());

                                let mut extended_fields = original.fields.clone();
                                extended_fields.append(&mut inter_ext.fields.clone());

                                let extended_interface = InterfaceType {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    fields: extended_fields,
                                };
                                type_map.insert(
                                    original_name.to_string(),
                                    GraphQLType::Interface(extended_interface),
                                );
                            }
                        }
                        None => return Err(String::from("The interface to extend is not found")),
                    }
                }
                graphql_parser::schema::TypeExtension::Union(union_ext) => {
                    let original_name = union_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_union) => {
                            if let GraphQLType::Union(original) = original_union {
                                let mut extended_directives = original.directives.clone();
                                extended_directives.append(&mut union_ext.directives.clone());

                                let mut extended_types = original.types.clone();
                                extended_types.append(&mut union_ext.types.clone());

                                let extended_union = UnionType {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    types: extended_types,
                                };
                                type_map.insert(
                                    original_name.to_string(),
                                    GraphQLType::Union(extended_union),
                                );
                            }
                        }
                        None => return Err(String::from("The union to extend is not found")),
                    }
                }
                graphql_parser::schema::TypeExtension::Enum(enum_ext) => {
                    let original_name = enum_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_enum) => {
                            if let GraphQLType::Enum(original) = original_enum {
                                let mut extended_directives = original.directives.clone();
                                extended_directives.append(&mut enum_ext.directives.clone());

                                let mut extended_values = original.values.clone();
                                extended_values.append(&mut enum_ext.values.clone());

                                let extended_enum = EnumType {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    values: extended_values,
                                };
                                type_map.insert(
                                    original_name.to_string(),
                                    GraphQLType::Enum(extended_enum),
                                );
                            }
                        }
                        None => return Err(String::from("The enum to extend is not found")),
                    }
                }
                graphql_parser::schema::TypeExtension::InputObject(input_ext) => {
                    let original_name = input_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_input) => {
                            if let GraphQLType::Input(original) = original_input {
                                let mut extended_directives = original.directives.clone();
                                extended_directives.append(&mut input_ext.directives.clone());

                                let mut extended_fields = original.fields.clone();
                                extended_fields.append(&mut input_ext.fields.clone());

                                let extended_input = InputObjectType {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    fields: extended_fields,
                                };
                                type_map.insert(
                                    original_name.to_string(),
                                    GraphQLType::Input(extended_input),
                                );
                            }
                        }
                        None => {
                            return Err(String::from("The input object to extend is not found"))
                        }
                    }
                }
            },
            graphql_parser::schema::Definition::DirectiveDefinition(directive) => {
                directive_map.insert(directive.name.to_string(), directive);
            }
        }
    }
    Ok(Schema {
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

        let query = schema.queries.get("repository").unwrap();
        println!("{:?}", query)
    }
}
