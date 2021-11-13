use std::collections::BTreeMap;

use graphql_parser::schema::Value;

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
