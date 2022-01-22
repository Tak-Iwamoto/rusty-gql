mod create_review;

use crate::graphql::*;
use rusty_gql::*;

pub struct Mutation;

#[Resolver]
impl Mutation {
    pub async fn createReview(
        &self,
        episode: Option<Episode>,
        review: ReviewInput,
    ) -> Option<Review> {
        create_review::createReview(episode, review).await
    }
}
