use crate::graphql::*;
use rusty_gql::*;

#[derive(Enum)]
pub enum Episode {
    NEWHOPE,
    EMPIRE,
    JEDI,
}