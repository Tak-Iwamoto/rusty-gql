# Union

rusty-gql defines GraphQL Union as Rust enum with different types and `#[derive(GqlUnion)]`.

src/graphql/resolver/search_result.rs

```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlUnion)]
pub enum SearchResult {
    Human(Human),
    Droid(Droid),
}
```

schema.graphql

```graphql
type Query {
  search(text: String): [SearchResult!]!
}

union SearchResult = Human | Droid

type Human {
  id: ID!
  name: String!
  homePlanet: String
}

type Droid {
  id: ID!
  name: String!
  primaryFunction: String
}
```
