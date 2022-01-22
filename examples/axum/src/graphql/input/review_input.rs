use crate::graphql::*;
use rusty_gql::*;

#[derive(InputObject)]
pub struct ReviewInput {
    stars: i64,
    commentary: Option<String>,
}
