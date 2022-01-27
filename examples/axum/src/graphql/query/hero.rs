#![allow(warnings, unused)]
use crate::{
    graphql::*,
    starwars::{han, luke, vader},
};
use rusty_gql::*;

pub async fn hero(episode: Option<Episode>) -> Option<Character> {
    match episode {
        Some(episode) => match episode {
            Episode::NEWHOPE => Some(Character::Human(luke())),
            Episode::EMPIRE => Some(Character::Human(han())),
            Episode::JEDI => Some(Character::Human(vader())),
        },
        None => None,
    }
}
