mod character;
mod droid;
mod reviews;
mod hero;
mod search;
mod human;

use crate::graphql::*;
use rusty_gql::*;

pub struct Query;

#[Resolver]
impl Query {
    pub async fn character(id: ID) -> Option<Character> {
        character::character(id).await
    }

    pub async fn droid(id: ID) -> Option<Droid> {
        droid::droid(id).await
    }

    pub async fn reviews(episode: Episode) -> Option<Vec<Option<Review>>> {
        reviews::reviews(episode).await
    }

    pub async fn hero(episode: Option<Episode>) -> Option<Character> {
        hero::hero(episode).await
    }

    pub async fn search(text: Option<String>, episode: Option<Episode>) -> Option<Vec<Option<SearchResult>>> {
        search::search(text,episode).await
    }

    pub async fn human(id: ID) -> Option<Human> {
        human::human(id).await
    }
}