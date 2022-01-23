mod droid;
mod character;
mod search;
mod human;
mod hero;
mod reviews;

use crate::graphql::*;
use rusty_gql::*;

#[derive(Clone)]
pub struct Query;

#[Resolver]
impl Query {
    pub async fn droid(&self, id: ID) -> Option<Droid> {
        droid::droid(id).await
    }

    pub async fn character(&self, id: ID) -> Option<Character> {
        character::character(id).await
    }

    pub async fn search(&self, text: Option<String>, episode: Option<Episode>) -> Vec<SearchResult> {
        search::search(text,episode).await
    }

    pub async fn human(&self, id: ID) -> Option<Human> {
        human::human(id).await
    }

    pub async fn hero(&self, episode: Option<Episode>) -> Option<Character> {
        hero::hero(episode).await
    }

    pub async fn reviews(&self, episode: Episode) -> Vec<Review> {
        reviews::reviews(episode).await
    }
}