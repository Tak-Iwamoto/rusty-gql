use std::collections::BTreeMap;

use rusty_gql::{GqlTypeDefinition, GqlValueType};

pub fn interface_ty_names(type_definitions: &BTreeMap<String, GqlTypeDefinition>) -> Vec<String> {
    type_definitions
        .iter()
        .filter(|(_, ty_def)| matches!(ty_def, GqlTypeDefinition::Interface(_)))
        .map(|(key, _)| key.clone())
        .collect::<Vec<_>>()
}

pub fn gql_value_ty_to_rust_ty(gql_value: &GqlValueType) -> String {
    value_ty_to_str(gql_value, true)
}

fn value_ty_to_str(gql_value: &GqlValueType, optional: bool) -> String {
    match gql_value {
        GqlValueType::NamedType(name) => gql_to_rust_type_str(name, optional),
        GqlValueType::ListType(list_type) => {
            if optional {
                format!("Option<Vec<{}>>", value_ty_to_str(list_type, true))
            } else {
                format!("Vec<{}>", value_ty_to_str(list_type, true))
            }
        }
        GqlValueType::NonNullType(non_null_type) => value_ty_to_str(non_null_type, false),
    }
}

fn gql_to_rust_type_str(gql_type: &str, optional: bool) -> String {
    let name = match gql_type {
        "Int" => "i64".to_string(),
        "Float" => "f64".to_string(),
        "String" => "String".to_string(),
        "Boolean" => "bool".to_string(),
        _ => gql_type.to_string(),
    };
    if optional {
        format!("Option<{}>", name)
    } else {
        name
    }
}

pub fn is_gql_primitive_ty(ty_name: &str) -> bool {
    reserved_scalar_names().contains(&ty_name)
}

fn reserved_scalar_names() -> Vec<&'static str> {
    vec!["String", "Int", "Float", "Boolean", "ID"]
}
