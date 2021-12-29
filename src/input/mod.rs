mod boolean;
mod id;
mod list;
mod number;
mod object;
mod optional;
mod string;

use crate::GqlValue;

pub trait GqlInputType: Send + Sync + Sized {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String>;

    fn to_gql_value(&self) -> GqlValue;
}
