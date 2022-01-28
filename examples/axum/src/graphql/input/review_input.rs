#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlInputObject)]
pub struct ReviewInput {
    pub stars: i32,
    pub commentary: Option<String>,
}
