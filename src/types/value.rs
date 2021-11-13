use std::collections::{BTreeMap, HashMap};

use graphql_parser::schema::Value;

use crate::GqlType;

#[derive(Debug, Clone)]
pub enum GqlValue {
    Variable(String),
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(String),
    List(Vec<Box<GqlValue>>),
    Object(BTreeMap<String, Box<GqlValue>>),
}

impl<'a> From<Value<'a, String>> for GqlValue {
    fn from(value: Value<'a, String>) -> Self {
        match value {
            Value::Variable(var) => GqlValue::Variable(var),
            Value::Int(int) => GqlValue::Int(int.as_i64().unwrap()),
            Value::Float(float) => GqlValue::Float(float),
            Value::String(string) => GqlValue::String(string),
            Value::Boolean(boolean) => GqlValue::Boolean(boolean),
            Value::Null => GqlValue::Null,
            Value::Enum(enum_value) => GqlValue::Enum(enum_value),
            Value::List(list) => {
                let result = list
                    .into_iter()
                    .map(|ele| Box::new(Self::from(ele)))
                    .collect();
                GqlValue::List(result)
            }
            Value::Object(obj) => {
                let mut result = BTreeMap::new();
                for (key, value) in obj {
                    result.insert(key, Box::new(Self::from(value)));
                }
                GqlValue::Object(result)
            }
        }
    }
}

pub fn value_from_ast<'a>(
    value: &Value<'a, String>,
    gql_type: &GqlType,
    variables: &Option<HashMap<String, GqlValue>>,
) -> GqlValue {
    match value {
        Value::Variable(variable) => {
            if let Some(vars) = variables {
                let variable_value = vars.get(&variable.to_string());
                if let GqlType::NonNull(_) = gql_type {
                    if variable_value.is_none() {
                        GqlValue::Null
                    } else {
                        variable_value.unwrap().clone()
                    }
                } else {
                    match variable_value {
                        Some(var) => var.clone(),
                        None => GqlValue::Null,
                    }
                }
            } else {
                GqlValue::Null
            }
        }
        Value::Int(int_value) => {
            if let Some(int) = int_value.as_i64() {
                GqlValue::Int(int)
            } else {
                GqlValue::Null
            }
        }
        Value::Float(float_value) => GqlValue::Float(*float_value),
        Value::String(str_value) => GqlValue::String(str_value.to_string()),
        Value::Boolean(bool_value) => GqlValue::Boolean(*bool_value),
        Value::Null => GqlValue::Null,
        Value::Enum(enum_literal) => GqlValue::Enum(enum_literal.to_string()),
        Value::List(list_value) => {
            let mut values = vec![];
            for item in list_value {
                let value = value_from_ast(item, &gql_type, variables);
                values.push(Box::new(value))
            }
            GqlValue::List(values)
        }
        Value::Object(obj) => {
            let mut obj_value = BTreeMap::new();
            for (k, v) in obj.iter() {
                let value = value_from_ast(&v, &gql_type, variables);
                obj_value.insert(k.to_string(), Box::new(value));
            }
            GqlValue::Object(obj_value)
        }
    }
}
