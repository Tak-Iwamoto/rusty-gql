use std::collections::BTreeMap;

use graphql_parser::schema::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GqlValue {
    Variable(String),
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(String),
    List(Vec<GqlValue>),
    Object(BTreeMap<String, GqlValue>),
}

impl Default for GqlValue {
    fn default() -> Self {
        GqlValue::Null
    }
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
                let result = list.into_iter().map(|ele| Self::from(ele)).collect();
                GqlValue::List(result)
            }
            Value::Object(obj) => {
                let mut result = BTreeMap::new();
                for (key, value) in obj {
                    result.insert(key, Self::from(value));
                }
                GqlValue::Object(result)
            }
        }
    }
}

impl From<String> for GqlValue {
    fn from(string_value: String) -> Self {
        GqlValue::String(string_value)
    }
}

impl From<i64> for GqlValue {
    fn from(int_value: i64) -> Self {
        GqlValue::Int(int_value)
    }
}

impl From<f64> for GqlValue {
    fn from(float_value: f64) -> Self {
        GqlValue::Float(float_value)
    }
}

impl From<bool> for GqlValue {
    fn from(bool_value: bool) -> Self {
        GqlValue::Boolean(bool_value)
    }
}

impl<T: Into<GqlValue>> From<Vec<T>> for GqlValue {
    fn from(list: Vec<T>) -> Self {
        GqlValue::List(list.into_iter().map(Into::into).collect())
    }
}

impl From<BTreeMap<String, GqlValue>> for GqlValue {
    fn from(obj: BTreeMap<String, GqlValue>) -> Self {
        GqlValue::Object(obj)
    }
}

// pub fn value_from_ast<'a>(
//     value: &Value<'a, String>,
//     gql_type: &GqlType,
//     variables: &Option<HashMap<String, GqlValue>>,
// ) -> GqlValue {
//     match value {
//         Value::Variable(variable) => {
//             if let Some(vars) = variables {
//                 let variable_value = vars.get(&variable.to_string());
//                 if let GqlType::NonNull(_) = gql_type {
//                     if variable_value.is_none() {
//                         GqlValue::Null
//                     } else {
//                         variable_value.unwrap().clone()
//                     }
//                 } else {
//                     match variable_value {
//                         Some(var) => var.clone(),
//                         None => GqlValue::Null,
//                     }
//                 }
//             } else {
//                 GqlValue::Null
//             }
//         }
//         Value::Int(int_value) => {
//             if let Some(int) = int_value.as_i64() {
//                 GqlValue::Int(int)
//             } else {
//                 GqlValue::Null
//             }
//         }
//         Value::Float(float_value) => GqlValue::Float(*float_value),
//         Value::String(str_value) => GqlValue::String(str_value.to_string()),
//         Value::Boolean(bool_value) => GqlValue::Boolean(*bool_value),
//         Value::Null => GqlValue::Null,
//         Value::Enum(enum_literal) => GqlValue::Enum(enum_literal.to_string()),
//         Value::List(list_value) => {
//             let mut values = vec![];
//             for item in list_value {
//                 let value = value_from_ast(item, &gql_type, variables);
//                 values.push(value);
//             }
//             GqlValue::List(values)
//         }
//         Value::Object(obj) => {
//             let mut obj_value = BTreeMap::new();
//             for (k, v) in obj.iter() {
//                 let value = value_from_ast(&v, &gql_type, variables);
//                 obj_value.insert(k.to_string(), value);
//             }
//             GqlValue::Object(obj_value)
//         }
//     }
// }
