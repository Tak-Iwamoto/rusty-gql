use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum GraphQLValue {
    Null,
    String(String),
    Boolean(bool),
    Int(i32),
    Float(f32),
    Enum(String),
    List(Vec<GraphQLValue>),
    Object(BTreeMap<String, GraphQLValue>),
}
