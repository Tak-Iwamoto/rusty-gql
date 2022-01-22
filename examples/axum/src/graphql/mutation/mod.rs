mod create_review;

use crate::graphql::*;
use rusty_gql::*;

pub struct Mutation;

impl Mutation {
    pub async fn createReview(episode: Option<Episode>, review: ReviewInput) -> Option<Review> {
        create_review::createReview(episode,review).await
    }
}