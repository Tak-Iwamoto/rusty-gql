use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlUnion, Debug, Clone)]
pub enum Character {
    Human(Human),
    Droid(Droid),
}
