mod serializer;
pub use serializer::serialize_into_gql_value;

use std::collections::BTreeMap;

use graphql_parser::schema::Value;
use serde::ser::Error as SerError;
use serde::{de::Visitor, Deserialize, Serialize, Serializer};
use serde_json::Number;

#[derive(Debug, Clone, Eq)]
pub enum GqlConstValue {
    Number(Number),
    String(String),
    Boolean(bool),
    Null,
    Enum(String),
    List(Vec<GqlValue>),
    Object(BTreeMap<String, GqlValue>),
}

impl GqlConstValue {
    pub fn to_value(&self) -> GqlValue {
        match self {
            GqlConstValue::Number(v) => GqlValue::Number(v.clone()),
            GqlConstValue::String(v) => GqlValue::String(v.clone()),
            GqlConstValue::Boolean(v) => GqlValue::Boolean(*v),
            GqlConstValue::Null => GqlValue::Null,
            GqlConstValue::Enum(v) => GqlValue::Enum(v.clone()),
            GqlConstValue::List(v) => GqlValue::List(v.clone()),
            GqlConstValue::Object(v) => GqlValue::Object(v.clone()),
        }
    }
}

impl PartialEq for GqlConstValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Enum(l0), Self::Enum(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => {
                if l0.len() != r0.len() {
                    return false;
                }
                l0.iter().zip(r0.iter()).all(|(l, r)| l == r)
            }
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub enum GqlValue {
    Variable(String),
    Number(Number),
    String(String),
    Boolean(bool),
    Null,
    Enum(String),
    List(Vec<GqlValue>),
    Object(BTreeMap<String, GqlValue>),
}

impl ToString for GqlValue {
    fn to_string(&self) -> String {
        match self {
            GqlValue::Variable(v) => v.clone(),
            GqlValue::Number(v) => v.to_string(),
            GqlValue::String(v) => v.clone(),
            GqlValue::Boolean(v) => v.to_string(),
            GqlValue::Null => "null".to_string(),
            GqlValue::Enum(v) => v.clone(),
            GqlValue::List(v) => format!("{:?}", v),
            GqlValue::Object(v) => format!("{:?}", v),
        }
    }
}

impl PartialEq for GqlValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Variable(l0), Self::Variable(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Enum(l0), Self::Enum(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => {
                if l0.len() != r0.len() {
                    return false;
                }
                l0.iter().zip(r0.iter()).all(|(l, r)| l == r)
            }
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Serialize for GqlValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            GqlValue::Variable(_) => Err(S::Error::custom("cannot serialize variable")),
            GqlValue::Number(v) => v.serialize(serializer),
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
        Ok(GqlValue::Number(v.into()))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GqlValue::Number(v.into()))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Number::from_f64(v) {
            Some(v) => Ok(GqlValue::Number(v)),
            None => Ok(GqlValue::Null),
        }
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
            Value::Int(int) => match int.as_i64() {
                Some(v) => GqlValue::Number(Number::from(v)),
                None => GqlValue::Null,
            },
            Value::Float(float) => match Number::from_f64(float) {
                Some(v) => GqlValue::Number(v),
                None => GqlValue::Null,
            },
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

macro_rules! from_integer {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for GqlValue {
                fn from(n: $ty) -> Self {
                    GqlValue::Number(n.into())
                }
            }
        )*
    };
}

from_integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl From<f32> for GqlValue {
    fn from(v: f32) -> Self {
        From::from(v as f64)
    }
}

impl From<f64> for GqlValue {
    fn from(v: f64) -> Self {
        Number::from_f64(v).map_or(GqlValue::Null, GqlValue::Number)
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
