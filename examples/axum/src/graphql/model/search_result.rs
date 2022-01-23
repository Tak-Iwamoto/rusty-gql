use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlUnion)]
pub enum SearchResult {
    Human(Human),
    Droid(Droid),
}
