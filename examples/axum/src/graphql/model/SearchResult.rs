use crate::graphql::*;
use rusty_gql::*;

#[derive(Union)]
pub enum SearchResult {
    Human(Human),
    Droid(Droid),
}