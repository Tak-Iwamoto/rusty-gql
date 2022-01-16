use crate::{GqlValue, ResolverResult};

pub trait CustomScalar: Send + Sized {
    fn from_gql_value(value: &GqlValue) -> Self;
    fn to_gql_value(&self) -> GqlValue;
}
