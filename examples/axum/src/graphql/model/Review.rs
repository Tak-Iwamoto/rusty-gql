use crate::graphql::*;
use rusty_gql::*;

pub struct Review {
    pub stars: i64,
    pub commentary: Option<String>,
}

#[Resolver]
impl Review {
    pub async fn episode(&self) -> Option<Episode> {
        todo!()
    }

    pub async fn stars(&self) -> i64 {
        self.stars
    }

    pub async fn commentary(&self) -> Option<String> {
        self.commentary.clone()
    }
}