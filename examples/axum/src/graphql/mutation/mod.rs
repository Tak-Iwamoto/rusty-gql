mod createReview;

use crate::graphql::*;
use rusty_gql::*;

pub struct Mutation;

impl Mutation {
    pub async fn createReview(episode: Option<Episode>, review: ReviewInput) -> Option<Review> {
        createReview::createReview(episode,review).await
    }
}