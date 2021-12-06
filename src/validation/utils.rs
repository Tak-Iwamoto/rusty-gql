use std::collections::HashSet;

use graphql_parser::{
    query::OperationDefinition,
    schema::{Type, Value},
    Pos,
};

use crate::{
    types::{GqlScalar, GqlValueType},
    GqlTypeDefinition, Schema,
};

use super::visitor::ValidationContext;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Scope<'a> {
    Operation(Option<&'a str>),
    Fragment(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DirectiveLocation {
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
}

fn check_arg_uniqueness(ctx: &mut ValidationContext<'_>, args: &Vec<(String, Value<'_, String>)>) {
    for (arg_name, arg_value) in args {}
}

pub fn check_valid_input_value(
    schema: &Schema,
    ty: &Type<'_, String>,
    value: &Value<'_, String>,
) -> Option<String> {
    match ty {
        Type::NamedType(type_name) => {
            if let Value::Null = value {
                return None;
            }
            let type_def = schema.type_definitions.get(type_name);
            match type_def {
                Some(def) => match def {
                    GqlTypeDefinition::Scalar(_) => {
                        if GqlScalar::is_valid_value(value) {
                            None
                        } else {
                            Some("Invalid type".to_string())
                        }
                    }
                    GqlTypeDefinition::InputObject(input_object) => match value {
                        Value::Object(object_value) => {
                            let mut value_keys: HashSet<String> =
                                object_value.keys().cloned().collect::<HashSet<String>>();

                            for field in &input_object.fields {
                                value_keys.remove(&field.name);
                                if let Some(value) = object_value.get(&field.name) {
                                    return check_valid_input_value(
                                        schema,
                                        &field.meta_type.to_parser_type(),
                                        value,
                                    );
                                } else if field.default_value.is_none()
                                    && matches!(field.meta_type, GqlValueType::NonNullType(_))
                                {
                                    return Some(format!(
                                        "field {} of type {} is non null type but not provided",
                                        &field.name, &input_object.name
                                    ));
                                }
                            }
                            if let Some(name) = value_keys.iter().next() {
                                return Some(format!(
                                    "unknown field {} of type {}",
                                    name, &input_object.name
                                ));
                            }
                            None
                        }
                        _ => None,
                    },
                    GqlTypeDefinition::Enum(enum_value) => match value {
                        Value::String(name) => {
                            if enum_value.contains(&name) {
                                Some(format!(
                                    "Enum type {} does not contain the value {}",
                                    enum_value.name, name
                                ))
                            } else {
                                None
                            }
                        }
                        Value::Enum(name) => {
                            if enum_value.contains(&name) {
                                Some(format!(
                                    "Enum type {} does not contain the value {}",
                                    enum_value.name, name
                                ))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    _ => None,
                },
                None => Some(format!("{} is not defined.", type_name)),
            }
        }
        Type::ListType(list_type) => match value {
            Value::Null => None,
            Value::List(values) => {
                for v in values {
                    let error_msg = check_valid_input_value(schema, &list_type, v);
                    if let Some(msg) = error_msg {
                        return Some(msg);
                    }
                }
                None
            }
            _ => check_valid_input_value(schema, &list_type, value),
        },
        Type::NonNullType(non_null_type) => match value {
            Value::Null => Some(format!(
                "type {} is non null but not provided value",
                get_type_name(ty)
            )),
            _ => check_valid_input_value(schema, &non_null_type, value),
        },
    }
}

pub fn is_sub_type(base: &Type<'_, String>, sub: &Type<'_, String>) -> bool {
    match (base, sub) {
        (Type::NonNullType(base_type), Type::NonNullType(sub_type)) => {
            is_sub_type(base_type, sub_type)
        }
        (Type::NamedType(base_type_name), Type::NonNullType(sub_type)) => {
            let sub_type_name = get_type_name(&sub_type);
            base_type_name.eq(&sub_type_name)
        }
        (Type::NamedType(base_type_name), Type::NamedType(sub_type_name)) => {
            base_type_name.eq(sub_type_name)
        }
        (Type::ListType(base_type), Type::ListType(sub_type)) => is_sub_type(base_type, sub_type),
        _ => false,
    }
}

pub fn get_type_name(ty: &Type<'_, String>) -> String {
    match ty {
        Type::NamedType(named_type) => named_type.to_string(),
        Type::ListType(list) => get_type_name(list),
        Type::NonNullType(non_null) => get_type_name(non_null),
    }
}

pub fn get_operation_name<'a>(operation: &OperationDefinition<'a, String>) -> Option<String> {
    match operation {
        OperationDefinition::SelectionSet(_) => None,
        OperationDefinition::Query(query) => query.name.clone(),
        OperationDefinition::Mutation(mutation) => mutation.name.clone(),
        OperationDefinition::Subscription(sub) => sub.name.clone(),
    }
}

pub fn referenced_variables<'a>(value: &'a Value<'a, String>) -> Vec<&'a str> {
    let mut vars = Vec::new();
    referenced_variables_to_vec(value, &mut vars);
    vars
}

fn referenced_variables_to_vec<'a>(value: &'a Value<'a, String>, vars: &mut Vec<&'a str>) {
    match value {
        Value::Variable(name) => {
            vars.push(name);
        }
        Value::List(values) => values
            .iter()
            .for_each(|value| referenced_variables_to_vec(value, vars)),
        Value::Object(obj) => obj
            .values()
            .for_each(|value| referenced_variables_to_vec(value, vars)),
        _ => {}
    }
}

pub fn get_operation_def_position<'a>(
    operation_definition: &OperationDefinition<'a, String>,
) -> Pos {
    match operation_definition {
        OperationDefinition::SelectionSet(selection_set) => selection_set.span.0,
        OperationDefinition::Query(query) => query.position,
        OperationDefinition::Mutation(mutation) => mutation.position,
        OperationDefinition::Subscription(subscription) => subscription.position,
    }
}
