use crate::{ResolverResult, Value};

pub trait CustomScalar: Sized {
    fn from_gql_value(value: &Value) -> ResolverResult<Self>;
    fn to_gql_value(&self) -> Value;
}
