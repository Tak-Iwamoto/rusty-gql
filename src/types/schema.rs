use std::{collections::HashMap, ops::Deref, sync::Arc};

use graphql_parser::schema::TypeDefinition;

use crate::{
    error::GqlError, CustomDirective, GqlDirective, GqlEnum, GqlInputObject, GqlInterface,
    GqlObject, GqlUnion,
};

use super::{
    argument::GqlArgument, directive::GqlDirectiveDefinition, field::GqlField, scalar::GqlScalar,
    type_definition::GqlTypeDefinition, GqlEnumValue,
};

pub struct SchemaInner {
    pub queries: HashMap<String, GqlField>,
    pub mutations: HashMap<String, GqlField>,
    pub subscriptions: HashMap<String, GqlField>,
    pub directives: HashMap<String, GqlDirectiveDefinition>,
    pub type_definitions: HashMap<String, GqlTypeDefinition>,
    pub interfaces: HashMap<String, GqlInterface>,
    pub query_type_name: String,
    pub mutation_type_name: String,
    pub subscription_type_name: String,
    pub custom_directives: HashMap<&'static str, Box<dyn CustomDirective>>,
}

pub struct Schema(Arc<SchemaInner>);

impl Schema {
    pub fn new(schema: SchemaInner) -> Self {
        Schema(Arc::new(schema))
    }
}

impl Deref for Schema {
    type Target = SchemaInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn build_schema(
    schema_documents: &[&str],
    custom_directives: HashMap<&'static str, Box<dyn CustomDirective>>,
) -> Result<Schema, GqlError> {
    let mut queries = HashMap::new();
    let mut mutations = HashMap::new();
    let mut subscriptions = HashMap::new();
    let mut type_definitions = HashMap::new();
    let mut directives = HashMap::new();
    let mut extensions = Vec::new();
    let mut schema_definition = None;
    let mut interfaces = HashMap::new();

    type_definitions.insert(
        "String".to_string(),
        GqlTypeDefinition::Scalar(GqlScalar::string_scalar()),
    );
    type_definitions.insert(
        "Int".to_string(),
        GqlTypeDefinition::Scalar(GqlScalar::int_scalar()),
    );
    type_definitions.insert(
        "Float".to_string(),
        GqlTypeDefinition::Scalar(GqlScalar::float_scalar()),
    );
    type_definitions.insert(
        "Boolean".to_string(),
        GqlTypeDefinition::Scalar(GqlScalar::boolean_scalar()),
    );
    type_definitions.insert(
        "ID".to_string(),
        GqlTypeDefinition::Scalar(GqlScalar::id_scalar()),
    );

    directives.insert("skip".to_string(), GqlDirectiveDefinition::skip_directive());
    directives.insert(
        "include".to_string(),
        GqlDirectiveDefinition::include_directive(),
    );
    directives.insert(
        "deprecated".to_string(),
        GqlDirectiveDefinition::deprecated_directive(),
    );

    for doc in schema_documents {
        let parsed_schema =
            graphql_parser::parse_schema::<String>(doc).expect("failed to parse graphql schema");
        for node in parsed_schema.definitions {
            match node {
                graphql_parser::schema::Definition::SchemaDefinition(schema_def) => {
                    schema_definition = Some(schema_def);
                }
                graphql_parser::schema::Definition::TypeDefinition(ty_def) => {
                    let gql_def = GqlTypeDefinition::from_schema_type_def(&ty_def);
                    type_definitions.insert(gql_def.name().to_string(), gql_def);

                    if let TypeDefinition::Interface(interface) = &ty_def {
                        interfaces.insert(
                            interface.name.to_string(),
                            GqlInterface::from(interface.clone()),
                        );
                    }
                }
                graphql_parser::schema::Definition::TypeExtension(ext) => {
                    extensions.push(ext);
                }
                graphql_parser::schema::Definition::DirectiveDefinition(directive) => {
                    let arguments = GqlArgument::from_vec_input_value(directive.arguments);
                    let result = GqlDirectiveDefinition {
                        position: directive.position,
                        name: directive.name,
                        description: directive.description,
                        arguments,
                        locations: directive.locations,
                    };
                    directives.insert(result.name.to_string(), result);
                }
            }
        }
    }

    for ext in extensions {
        match ext {
            graphql_parser::schema::TypeExtension::Scalar(scalar_ext) => {
                let original_name = scalar_ext.name.clone();
                match type_definitions.get(&original_name) {
                    Some(original_scalar) => {
                        if let GqlTypeDefinition::Scalar(original) = original_scalar {
                            let mut extended_directives = original.directives.clone();
                            let directives =
                                GqlDirective::from_vec_directive(scalar_ext.directives);
                            extended_directives.extend(directives);

                            let extended_scalar = GqlScalar {
                                position: original.position,
                                description: original.description.clone(),
                                name: original_name.clone(),
                                directives: extended_directives,
                            };
                            type_definitions
                                .insert(original_name, GqlTypeDefinition::Scalar(extended_scalar));
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
                match type_definitions.get(&original_name) {
                    Some(original_obj) => {
                        if let GqlTypeDefinition::Object(original) = original_obj {
                            let mut extended_directives = original.directives.clone();
                            let directives = GqlDirective::from_vec_directive(obj_ext.directives);
                            extended_directives.extend(directives);

                            let mut extended_fields = original.fields.clone();
                            let fields = GqlField::from_vec_field(obj_ext.fields);
                            extended_fields.extend(fields);

                            let mut extended_impl_interfaces =
                                original.implements_interfaces.clone();
                            extended_impl_interfaces.extend(obj_ext.implements_interfaces.clone());

                            let extended_obj = GqlObject {
                                position: original.position,
                                description: original.description.clone(),
                                name: original_name.clone(),
                                directives: extended_directives,
                                fields: extended_fields,
                                implements_interfaces: extended_impl_interfaces,
                            };
                            type_definitions.insert(
                                original_name.to_string(),
                                GqlTypeDefinition::Object(extended_obj),
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
                match type_definitions.get(&original_name) {
                    Some(original_interface) => {
                        if let GqlTypeDefinition::Interface(original) = original_interface {
                            let mut extended_directives = original.directives.clone();
                            let directives = GqlDirective::from_vec_directive(inter_ext.directives);
                            extended_directives.extend(directives);

                            let mut extended_fields = original.fields.clone();
                            let fields = GqlField::from_vec_field(inter_ext.fields);
                            extended_fields.extend(fields);

                            let extended_interface = GqlInterface {
                                position: original.position,
                                description: original.description.clone(),
                                name: original_name.clone(),
                                directives: extended_directives,
                                fields: extended_fields,
                            };
                            type_definitions.insert(
                                original_name.to_string(),
                                GqlTypeDefinition::Interface(extended_interface.clone()),
                            );
                            interfaces
                                .insert(original_name.to_string(), extended_interface.clone());
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
                match type_definitions.get(&original_name) {
                    Some(original_union) => {
                        if let GqlTypeDefinition::Union(original) = original_union {
                            let mut extended_directives = original.directives.clone();
                            let directives =
                                GqlDirective::from_vec_directive(union_ext.directives.clone());
                            extended_directives.extend(directives);

                            let mut extended_types = original.types.clone();
                            extended_types.extend(union_ext.types.clone());

                            let extended_union = GqlUnion {
                                position: original.position,
                                description: original.description.clone(),
                                name: original_name.clone(),
                                directives: extended_directives,
                                types: extended_types,
                            };
                            type_definitions.insert(
                                original_name.to_string(),
                                GqlTypeDefinition::Union(extended_union),
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
                match type_definitions.get(&original_name) {
                    Some(original_enum) => {
                        if let GqlTypeDefinition::Enum(original) = original_enum {
                            let mut extended_directives = original.directives.clone();
                            let directives =
                                GqlDirective::from_vec_directive(enum_ext.directives.clone());
                            extended_directives.extend(directives);

                            let mut extended_values = original.values.clone();
                            let values: Vec<GqlEnumValue> = enum_ext
                                .values
                                .into_iter()
                                .map(|value| GqlEnumValue::from(value))
                                .collect();
                            extended_values.extend(values);

                            let extended_enum = GqlEnum {
                                position: original.position,
                                description: original.description.clone(),
                                name: original_name.clone(),
                                directives: extended_directives,
                                values: extended_values,
                            };
                            let gql_enum = GqlEnum::from(extended_enum);
                            type_definitions.insert(
                                original_name.to_string(),
                                GqlTypeDefinition::Enum(gql_enum),
                            );
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
                match type_definitions.get(&original_name) {
                    Some(original_input) => {
                        if let GqlTypeDefinition::InputObject(original) = original_input {
                            let mut extended_directives = original.directives.clone();
                            let directives =
                                GqlDirective::from_vec_directive(input_ext.directives.clone());
                            extended_directives.extend(directives);

                            let mut extended_fields = original.fields.clone();
                            let fields = GqlArgument::from_vec_input_value(input_ext.fields);
                            extended_fields.extend(fields);

                            let extended_input = GqlInputObject {
                                position: original.position,
                                description: original.description.clone(),
                                name: original_name.clone(),
                                directives: extended_directives,
                                fields: extended_fields,
                            };
                            type_definitions.insert(
                                original_name.to_string(),
                                GqlTypeDefinition::InputObject(extended_input),
                            );
                        }
                    }
                    None => {
                        return Err(GqlError::new(
                            format!("The {} input object to extend is not found", original_name),
                            None,
                        ))
                    }
                }
            }
        }
    }

    let mut query_type_name = "Query".to_string();
    let mut mutation_type_name = "Mutation".to_string();
    let mut subscription_type_name = "Subscription".to_string();

    if let Some(def) = schema_definition {
        if let Some(query) = def.query {
            query_type_name = query;
        }
        if let Some(mutation) = def.mutation {
            mutation_type_name = mutation;
        }
        if let Some(subscription) = def.subscription {
            subscription_type_name = subscription;
        }
    }

    match type_definitions.get(&query_type_name) {
        Some(query_def) => {
            if let GqlTypeDefinition::Object(def) = query_def {
                for f in &def.fields {
                    queries.insert(f.name.to_string(), GqlField::from(f.clone()));
                }
            }
        }
        None => {
            return Err(GqlError::new("Query type must be defined", None));
        }
    }

    if let Some(GqlTypeDefinition::Object(mutation_def)) = type_definitions.get(&mutation_type_name)
    {
        for f in &mutation_def.fields {
            mutations.insert(f.name.to_string(), GqlField::from(f.clone()));
        }
    }

    if let Some(GqlTypeDefinition::Object(subscription_def)) =
        type_definitions.get(&subscription_type_name)
    {
        for f in &subscription_def.fields {
            subscriptions.insert(f.name.to_string(), GqlField::from(f.clone()));
        }
    }

    Ok(Schema(Arc::new(SchemaInner {
        queries,
        mutations,
        subscriptions,
        directives,
        type_definitions,
        query_type_name,
        mutation_type_name,
        subscription_type_name,
        interfaces,
        custom_directives,
    })))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::build_schema;

    #[test]
    fn it_works() {
        let contents = fs::read_to_string("tests/schemas/github.graphql");
        let schema = build_schema(&vec![contents.unwrap().as_str()], Default::default()).unwrap();

        assert!(schema.queries.get("repository").is_some());
        assert!(schema.type_definitions.get("AddCommentInput").is_some());

        let base = fs::read_to_string("tests/schemas/test_schema.graphql").unwrap();
        let extend = fs::read_to_string("tests/schemas/extend.graphql").unwrap();
        let schema =
            build_schema(&vec![base.as_str(), extend.as_str()], Default::default()).unwrap();

        assert!(schema.queries.get("pets").is_some());
        assert!(schema.queries.get("authors").is_some());
    }
}
