use crate::graphql::*;
use rusty_gql::*;

pub async fn createReview(episode: Option<Episode>, review: ReviewInput) -> Option<Review> {
    Some(Review {
        stars: review.stars,
        commentary: review.commentary,
        episode,
    })
}
