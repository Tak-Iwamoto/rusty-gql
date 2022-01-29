# InputObject

rusty-gql defines GraphQL InputObject as Rust struct with `#[derive(GqlInputObject)]`.

schema.graphql

```graphql
type Mutation {
  createReview(episode: Episode, review: ReviewInput!): Review
}

input ReviewInput {
  stars: Int!
  commentary: String
}

type Review {
  episode: Episode
  stars: Int!
  commentary: String
}
```

input/review_input.rs

```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlInputObject)]
pub struct ReviewInput {
    pub stars: i32,
    pub commentary: Option<String>,
}
```
