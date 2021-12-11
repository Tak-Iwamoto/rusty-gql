use std::collections::BTreeMap;

use graphql_parser::schema::Value;
use serde::ser::Error as SerError;
use serde::{de::Visitor, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone)]
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

impl Serialize for GqlValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            GqlValue::Variable(_) => Err(S::Error::custom("cannot serialize variable")),
            GqlValue::Int(v) => serializer.serialize_i64(*v),
            GqlValue::Float(v) => serializer.serialize_f64(*v),
            GqlValue::String(v) => serializer.serialize_str(v),
            GqlValue::Boolean(v) => serializer.serialize_bool(*v),
            GqlValue::Null => serializer.serialize_none(),
            GqlValue::Enum(v) => serializer.serialize_str(v),
            GqlValue::List(v) => v.serialize(serializer),
            GqlValue::Object(v) => v.serialize(serializer),
        }
    }
}

struct GqlValueVisitor;

impl<'de> Visitor<'de> for GqlValueVisitor {
    type Value = GqlValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("GraphQL value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GqlValue::Boolean(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GqlValue::Int(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GqlValue::Float(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GqlValue::String(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GqlValue::String(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GqlValue::Null)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = seq.next_element()? {
            vec.push(elem);
        }
        Ok(GqlValue::List(vec))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut result = BTreeMap::new();
        while let Some((name, value)) = map.next_entry()? {
            result.insert(name, value);
        }
        Ok(GqlValue::Object(result))
    }
}

impl<'de> Deserialize<'de> for GqlValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(GqlValueVisitor)
    }
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
