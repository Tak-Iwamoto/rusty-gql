use crate::graphql::*;
use rusty_gql::*;

pub struct Review {
    stars: i64,
    commentary: Option<String>,
}

#[Resolver]
impl Review {
    async fn episode(&self) -> Option<Episode> {
        todo!()
    }

    async fn stars(&self) -> i64 {
        self.stars
    }

    async fn commentary(&self) -> Option<String> {
        self.commentary.clone()
    }
}