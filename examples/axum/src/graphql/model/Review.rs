#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

pub struct Review {
    pub stars: i32,
    pub commentary: Option<String>,
    pub episode: Option<Episode>,
}

#[GqlType]
impl Review {
    pub async fn episode(&self, ctx: &Context<'_>) -> Option<Episode> {
        self.episode
    }

    pub async fn stars(&self, ctx: &Context<'_>) -> i32 {
        self.stars
    }

    pub async fn commentary(&self, ctx: &Context<'_>) -> Option<String> {
        self.commentary.clone()
    }
}
