use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlInputObject)]
pub struct ReviewInput {
    stars: i64,
    commentary: Option<String>,
}
