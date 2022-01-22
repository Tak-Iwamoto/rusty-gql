use crate::graphql::*;
use rusty_gql::*;

#[derive(Enum)]
pub enum LengthUnit {
    METER,
    FOOT,
}