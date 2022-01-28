#![allow(warnings, unused)]
use crate::{
    graphql::*,
    starwars::{c3po, han, leia, luke, r2d2, vader},
};
use rusty_gql::*;

pub async fn character(ctx: &Context<'_>, id: ID) -> Option<Character> {
    if id.0 == "1" {
        Some(Character::Human(luke()))
    } else if id.0 == "2" {
        Some(Character::Human(vader()))
    } else if id.0 == "3" {
        Some(Character::Human(han()))
    } else if id.0 == "4" {
        Some(Character::Human(leia()))
    } else if id.0 == "5" {
        Some(Character::Droid(r2d2()))
    } else if id.0 == "6" {
        Some(Character::Droid(c3po()))
    } else {
        None
    }
}
