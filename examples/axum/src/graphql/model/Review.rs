use crate::graphql::*;
use rusty_gql::*;

pub struct Review {
    pub stars: i64,
    pub commentary: Option<String>,
    pub episode: Option<Episode>,
}

#[GqlType]
impl Review {
    pub async fn episode(&self) -> Option<Episode> {
        self.episode
    }

    pub async fn stars(&self) -> i64 {
        self.stars
    }

    pub async fn commentary(&self) -> Option<String> {
        self.commentary.clone()
    }
}
