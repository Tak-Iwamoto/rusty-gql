mod boolean;
mod id;
mod list;
mod number;
mod object;
mod optional;
mod string;

use crate::GqlValue;

pub trait VariableType: Send + Sync + Sized {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String>;

    fn into_gql_value(&self) -> GqlValue;
}
