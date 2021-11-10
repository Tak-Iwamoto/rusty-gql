use crate::types::GraphQLType;
use graphql_parser::schema::Value;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, PartialEq)]
pub enum GraphQLValue {
    Null,
    String(String),
    Boolean(bool),
    Int(i64),
    Float(f64),
    Enum(String),
    List(Vec<GraphQLValue>),
    Object(BTreeMap<String, GraphQLValue>),
}

pub fn value_from_ast<'a>(
    value: &Value<'a, String>,
    gql_type: &GraphQLType,
    variables: &Option<HashMap<String, GraphQLValue>>,
) -> GraphQLValue {
    match value {
        Value::Variable(variable) => {
            if let Some(vars) = variables {
                let variable_value = vars.get(&variable.to_string());
                if let GraphQLType::NonNull(_) = gql_type {
                    if variable_value.is_none() {
                        GraphQLValue::Null
                    } else {
                        variable_value.unwrap().clone()
                    }
                } else {
                    match variable_value {
                        Some(var) => var.clone(),
                        None => GraphQLValue::Null,
                    }
                }
            } else {
                GraphQLValue::Null
            }
        }
        Value::Int(int_value) => {
            if let Some(int) = int_value.as_i64() {
                GraphQLValue::Int(int)
            } else {
                GraphQLValue::Null
            }
        }
        Value::Float(float_value) => GraphQLValue::Float(*float_value),
        Value::String(str_value) => GraphQLValue::String(str_value.to_string()),
        Value::Boolean(bool_value) => GraphQLValue::Boolean(*bool_value),
        Value::Null => GraphQLValue::Null,
        Value::Enum(enum_literal) => GraphQLValue::Enum(enum_literal.to_string()),
        Value::List(list_value) => {
            let mut values = vec![];
            for item in list_value {
                let value = value_from_ast(item, &gql_type, variables);
                values.push(value)
            }
            GraphQLValue::List(values)
        }
        Value::Object(obj) => {
            let mut obj_value = BTreeMap::new();
            for (k, v) in obj.iter() {
                let value = value_from_ast(&v, &gql_type, variables);
                obj_value.insert(k.to_string(), value);
            }
            GraphQLValue::Object(obj_value)
        }
    }
}
