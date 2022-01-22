mod boolean;
mod id;
mod list;
mod number;
mod object;
mod optional;
mod string;

use std::sync::Arc;

use crate::GqlValue;

pub trait VariableType: Send + Sync + Sized {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String>;

    fn into_gql_value(&self) -> GqlValue;
}

impl<T: VariableType> VariableType for Arc<T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        T::from_gql_value(value).map(|v| Arc::new(v))
    }

    fn into_gql_value(&self) -> GqlValue {
        T::into_gql_value(&self)
    }
}

impl<T: VariableType> VariableType for Box<T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        T::from_gql_value(value).map(|v| Box::new(v))
    }

    fn into_gql_value(&self) -> GqlValue {
        T::into_gql_value(&self)
    }
}
