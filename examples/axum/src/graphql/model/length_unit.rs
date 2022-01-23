use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlEnum)]
pub enum LengthUnit {
    METER,
    FOOT,
}
