use crate::graphql::*;
use rusty_gql::*;

#[derive(Scalar)]
pub struct DateTime;

impl VariableType for DateTime {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        todo!()
    }

    fn into_gql_value(&self) -> GqlValue {
        todo!()
    }
}
