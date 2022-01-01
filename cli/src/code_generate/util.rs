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

fn value_ty_to_str(gql_value: &GqlValueType, root: bool) -> String {
    match gql_value {
        GqlValueType::NamedType(name) => gql_to_rust_type_str(name, root),
        GqlValueType::ListType(list_type) => {
            format!("Vec<{}>", value_ty_to_str(list_type, false))
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
