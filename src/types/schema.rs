use std::{collections::BTreeMap, ops::Deref, sync::Arc};

use crate::error::GqlError;

use super::{
    argument::GqlArgument,
    directive::{GqlDirective, GqlDirectiveDefinition},
    field::GqlField,
    gql_enum::{GqlEnum, GqlEnumValue},
    gql_type::GqlMetaType,
    gql_union::GqlUnion,
    input_object::GqlInputObject,
    interface::GqlInterface,
    object::GqlObject,
    scalar::GqlScalar,
};

#[derive(Debug)]
pub struct Schema {
    pub queries: BTreeMap<String, GqlField>,
    pub mutations: BTreeMap<String, GqlField>,
    pub subscriptions: BTreeMap<String, GqlField>,
    pub directives: BTreeMap<String, GqlDirectiveDefinition>,
    pub type_map: BTreeMap<String, GqlMetaType>,
    pub query_root_type: Option<String>,
    pub mutation_root_type: Option<String>,
    pub subscription_root_type: Option<String>,
}

#[derive(Debug)]
pub struct ArcSchema(Arc<Schema>);

impl ArcSchema {
    pub fn new(schema: Schema) -> Self {
        ArcSchema(Arc::new(schema))
    }
}

impl Deref for ArcSchema {
    type Target = Schema;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn build_schema(schema_doc: &str) -> Result<Schema, GqlError> {
    let parsed_schema =
        graphql_parser::parse_schema::<String>(schema_doc).expect("failed to parse graphql schema");
    let mut query_map = BTreeMap::new();
    let mut mutation_map = BTreeMap::new();
    let mut subscription_map = BTreeMap::new();
    let mut type_map = BTreeMap::new();
    let mut directive_map = BTreeMap::new();
    let mut query_root_type = None;
    let mut mutation_root_type = None;
    let mut subscription_root_type = None;

    for node in parsed_schema.definitions {
        match node {
            graphql_parser::schema::Definition::SchemaDefinition(schema) => {
                query_root_type = schema.query;
                mutation_root_type = schema.mutation;
                subscription_root_type = schema.subscription;
            }
            graphql_parser::schema::Definition::TypeDefinition(type_def) => match type_def {
                graphql_parser::schema::TypeDefinition::Scalar(scalar) => {
                    let gql_scalar = GqlScalar::from(scalar);
                    type_map.insert(gql_scalar.name.to_string(), GqlMetaType::Scalar(gql_scalar));
                }

                graphql_parser::schema::TypeDefinition::Object(obj) => match &*obj.name {
                    "Query" => {
                        for f in obj.fields {
                            query_map.insert(f.name.to_string(), GqlField::from(f));
                        }
                    }
                    "Mutation" => {
                        for f in obj.fields {
                            mutation_map.insert(f.name.to_string(), GqlField::from(f));
                        }
                    }
                    "Subscription" => {
                        for f in obj.fields {
                            subscription_map.insert(f.name.to_string(), GqlField::from(f));
                        }
                    }
                    _ => {
                        let gql_object = GqlObject::from(obj);
                        type_map.insert(gql_object.name.to_string(), GqlMetaType::Object(gql_object));
                    }
                },
                graphql_parser::schema::TypeDefinition::Interface(interface) => {
                    let gql_interface = GqlInterface::from(interface);
                    type_map.insert(
                        gql_interface.name.to_string(),
                        GqlMetaType::Interface(gql_interface),
                    );
                }
                graphql_parser::schema::TypeDefinition::Union(uni) => {
                    let gql_union = GqlUnion::from(uni);
                    type_map.insert(gql_union.name.to_string(), GqlMetaType::Union(gql_union));
                }
                graphql_parser::schema::TypeDefinition::Enum(enum_type) => {
                    let gql_enum = GqlEnum::from(enum_type);
                    type_map.insert(gql_enum.name.to_string(), GqlMetaType::Enum(gql_enum));
                }
                graphql_parser::schema::TypeDefinition::InputObject(input) => {
                    let gql_input_obj = GqlInputObject::from(input);
                    type_map.insert(
                        gql_input_obj.name.to_string(),
                        GqlMetaType::Input(gql_input_obj),
                    );
                }
            },
            graphql_parser::schema::Definition::TypeExtension(type_ext) => match type_ext {
                graphql_parser::schema::TypeExtension::Scalar(scalar_ext) => {
                    let original_name = scalar_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_scalar) => {
                            if let GqlMetaType::Scalar(original) = original_scalar {
                                let mut extended_directives = original.directives.clone();
                                let mut directives =
                                    GqlDirective::from_vec_directive(scalar_ext.directives);
                                extended_directives.append(&mut directives);

                                let extended_scalar = GqlScalar {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                };
                                type_map.insert(original_name, GqlMetaType::Scalar(extended_scalar));
                            }
                        }
                        None => {
                            return Err(GqlError::new(
                                format!("The {} scalar to extend is not found", original_name),
                                None,
                            ))
                        }
                    }
                }
                graphql_parser::schema::TypeExtension::Object(obj_ext) => {
                    let original_name = obj_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_obj) => {
                            if let GqlMetaType::Object(original) = original_obj {
                                let mut extended_directives = original.directives.clone();
                                let mut directives =
                                    GqlDirective::from_vec_directive(obj_ext.directives);
                                extended_directives.append(&mut directives);

                                let mut extended_fields = original.fields.clone();
                                let mut fields = GqlField::from_vec_field(obj_ext.fields);
                                extended_fields.append(&mut fields);

                                let mut extended_impl_interfaces =
                                    original.implements_interfaces.clone();
                                extended_impl_interfaces
                                    .append(&mut obj_ext.implements_interfaces.clone());

                                let extended_obj = GqlObject {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    fields: extended_fields,
                                    implements_interfaces: extended_impl_interfaces,
                                };
                                type_map.insert(
                                    original_name.to_string(),
                                    GqlMetaType::Object(extended_obj),
                                );
                            }
                        }
                        None => {
                            return Err(GqlError::new(
                                format!("The {} object to extend is not found", original_name),
                                None,
                            ))
                        }
                    }
                }
                graphql_parser::schema::TypeExtension::Interface(inter_ext) => {
                    let original_name = inter_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_interface) => {
                            if let GqlMetaType::Interface(original) = original_interface {
                                let mut extended_directives = original.directives.clone();
                                let mut directives =
                                    GqlDirective::from_vec_directive(inter_ext.directives);
                                extended_directives.append(&mut directives);

                                let mut extended_fields = original.fields.clone();
                                let mut fields = GqlField::from_vec_field(inter_ext.fields);
                                extended_fields.append(&mut fields);

                                let extended_interface = GqlInterface {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    fields: extended_fields,
                                };
                                type_map.insert(
                                    original_name.to_string(),
                                    GqlMetaType::Interface(extended_interface),
                                );
                            }
                        }
                        None => {
                            return Err(GqlError::new(
                                format!("The {} interface to extend is not found", original_name),
                                None,
                            ))
                        }
                    }
                }
                graphql_parser::schema::TypeExtension::Union(union_ext) => {
                    let original_name = union_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_union) => {
                            if let GqlMetaType::Union(original) = original_union {
                                let mut extended_directives = original.directives.clone();
                                let mut directives =
                                    GqlDirective::from_vec_directive(union_ext.directives.clone());
                                extended_directives.append(&mut directives);

                                let mut extended_types = original.types.clone();
                                extended_types.append(&mut union_ext.types.clone());

                                let extended_union = GqlUnion {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    types: extended_types,
                                };
                                type_map.insert(
                                    original_name.to_string(),
                                    GqlMetaType::Union(extended_union),
                                );
                            }
                        }
                        None => {
                            return Err(GqlError::new(
                                format!("The {} union to extend is not found", original_name),
                                None,
                            ))
                        }
                    }
                }
                graphql_parser::schema::TypeExtension::Enum(enum_ext) => {
                    let original_name = enum_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_enum) => {
                            if let GqlMetaType::Enum(original) = original_enum {
                                let mut extended_directives = original.directives.clone();
                                let mut directives =
                                    GqlDirective::from_vec_directive(enum_ext.directives.clone());
                                extended_directives.append(&mut directives);

                                let mut extended_values = original.values.clone();
                                let mut values = enum_ext
                                    .values
                                    .into_iter()
                                    .map(|value| GqlEnumValue::from(value))
                                    .collect();
                                extended_values.append(&mut values);

                                let extended_enum = GqlEnum {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    values: extended_values,
                                };
                                let gql_enum = GqlEnum::from(extended_enum);
                                type_map.insert(original_name.to_string(), GqlMetaType::Enum(gql_enum));
                            }
                        }
                        None => {
                            return Err(GqlError::new(
                                format!("The {} enum to extend is not found", original_name),
                                None,
                            ))
                        }
                    }
                }
                graphql_parser::schema::TypeExtension::InputObject(input_ext) => {
                    let original_name = input_ext.name.clone();
                    match type_map.get(&original_name) {
                        Some(original_input) => {
                            if let GqlMetaType::Input(original) = original_input {
                                let mut extended_directives = original.directives.clone();
                                let mut directives =
                                    GqlDirective::from_vec_directive(input_ext.directives.clone());
                                extended_directives.append(&mut directives);

                                let mut extended_fields = original.fields.clone();
                                let mut fields =
                                    GqlArgument::from_vec_input_value(input_ext.fields);
                                extended_fields.append(&mut fields);

                                let extended_input = GqlInputObject {
                                    position: original.position,
                                    description: original.description.clone(),
                                    name: original_name.clone(),
                                    directives: extended_directives,
                                    fields: extended_fields,
                                };
                                type_map.insert(
                                    original_name.to_string(),
                                    GqlMetaType::Input(extended_input),
                                );
                            }
                        }
                        None => {
                            return Err(GqlError::new(
                                format!(
                                    "The {} input object to extend is not found",
                                    original_name
                                ),
                                None,
                            ))
                        }
                    }
                }
            },
            graphql_parser::schema::Definition::DirectiveDefinition(directive) => {
                let arguments = GqlArgument::from_vec_input_value(directive.arguments);
                let result = GqlDirectiveDefinition {
                    position: directive.position,
                    name: directive.name,
                    description: directive.description,
                    arguments,
                };
                directive_map.insert(result.name.to_string(), result);
            }
        }
    }
    Ok(Schema {
        queries: query_map,
        mutations: mutation_map,
        subscriptions: subscription_map,
        directives: directive_map,
        type_map,
        query_root_type,
        mutation_root_type,
        subscription_root_type,
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
