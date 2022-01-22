use crate::graphql::*;
use rusty_gql::*;

pub struct ReviewInput {
    stars: i64,
    commentary: Option<String>,
}