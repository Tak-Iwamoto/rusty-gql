use crate::graphql::*;
use rusty_gql::*;

#[derive(Union)]
pub enum Character {
    Droid(Droid),
    Human(Human),
}