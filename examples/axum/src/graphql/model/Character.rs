use crate::graphql::*;
use rusty_gql::*;

#[derive(Union, Debug, Clone)]
pub enum Character {
    Human(Human),
    Droid(Droid),
}
