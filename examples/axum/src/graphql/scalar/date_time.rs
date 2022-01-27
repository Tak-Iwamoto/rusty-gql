#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlScalar)]
pub struct DateTime;

impl GqlInputType for DateTime {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        todo!()
    }

    fn into_gql_value(&self) -> GqlValue {
        todo!()
    }
}
