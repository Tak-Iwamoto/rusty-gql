use crate::graphql::*;
use rusty_gql::*;

#[derive(Union)]
pub enum Character {
    Human(Human),
    Droid(Droid),
}