#![allow(warnings, unused)]
use crate::{graphql::*, starwars::all_reviews};
use rusty_gql::*;

pub async fn reviews(episode: Episode) -> Vec<Review> {
    all_reviews()
}
