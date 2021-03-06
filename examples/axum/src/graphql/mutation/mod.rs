#![allow(warnings, unused)]
mod create_review;

use crate::graphql::*;
use rusty_gql::*;

#[derive(Clone)]
pub struct Mutation;

#[GqlType]
impl Mutation {
    pub async fn createReview(
        &self,
        episode: Option<Episode>,
        review: ReviewInput,
    ) -> Option<Review> {
        create_review::createReview(episode, review).await
    }
}
