# Enum

rusty-gql defines GraphQL Enum as Rust enum with `#[derive(GqlEnum)]`.

src/graphql/resolver/episode.rs

```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlEnum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Episode {
    NEWHOPE,
    EMPIRE,
    JEDI,
}
```

schema.graphql

```graphql
enum Episode {
  NEWHOPE
  EMPIRE
  JEDI
}
```
