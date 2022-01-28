#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

mod character;
mod droid;
mod hero;
mod human;
mod reviews;
mod search;

#[derive(Clone)]
pub struct Query;

#[GqlType]
impl Query {
    pub async fn droid(&self, ctx: &Context<'_>, id: ID) -> Option<Droid> {
        droid::droid(ctx, id).await
    }

    pub async fn character(&self, ctx: &Context<'_>, id: ID) -> Option<Character> {
        character::character(ctx, id).await
    }

    pub async fn search(
        &self,
        ctx: &Context<'_>,
        text: Option<String>,
        episode: Option<Episode>,
    ) -> Vec<SearchResult> {
        search::search(ctx, text, episode).await
    }

    pub async fn human(&self, ctx: &Context<'_>, id: ID) -> Option<Human> {
        human::human(ctx, id).await
    }

    pub async fn hero(&self, ctx: &Context<'_>, episode: Option<Episode>) -> Option<Character> {
        hero::hero(ctx, episode).await
    }

    pub async fn reviews(&self, ctx: &Context<'_>, episode: Episode) -> Vec<Review> {
        reviews::reviews(ctx, episode).await
    }
}
