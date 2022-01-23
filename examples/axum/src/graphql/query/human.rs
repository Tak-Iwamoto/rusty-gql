use crate::{
    graphql::*,
    starwars::{han, leia, luke, vader},
};
use rusty_gql::*;

pub async fn human(id: ID) -> Option<Human> {
    if id.0 == "1" {
        Some(luke())
    } else if id.0 == "2" {
        Some(vader())
    } else if id.0 == "3" {
        Some(han())
    } else if id.0 == "4" {
        Some(leia())
    } else {
        None
    }
}
