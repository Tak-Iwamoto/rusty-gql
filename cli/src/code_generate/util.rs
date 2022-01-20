use std::collections::HashMap;

use rusty_gql::{GqlTypeDefinition, GqlValueType};

pub fn get_interface_impl_object_map(
    type_definitions: &HashMap<String, GqlTypeDefinition>,
) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for (_, ty_def) in type_definitions {
        if let GqlTypeDefinition::Object(obj) = ty_def {
            for interface_name in &obj.implements_interfaces {
                map.entry(interface_name.to_string())
                    .or_insert_with(|| vec![])
                    .push(obj.name.to_string());
            }
        }
    }
    map
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

pub fn is_gql_primitive_ty(type_name: &str) -> bool {
    vec!["String", "Int", "Float", "Boolean", "ID"].contains(&type_name)
}

pub fn is_introspection_type_names(type_name: &str) -> bool {
    vec![
        "__Directive",
        "__DirectiveLocation",
        "__EnumValue",
        "__Field",
        "__InputValue",
        "__Schema",
        "__Type",
        "__TypeKind",
    ]
    .contains(&type_name)
}
