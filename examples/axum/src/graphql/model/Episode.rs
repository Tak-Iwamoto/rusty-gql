use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlEnum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Episode {
    NEWHOPE,
    EMPIRE,
    JEDI,
}
