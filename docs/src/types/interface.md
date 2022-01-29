# Interface

rusty-gql defines Graphql Interface as Rust enum with different types.

schema.graphql
```graphql
interface Pet {
  name: String
}

type Cat implements Pet {
  name: String
  meows: Boolean
}

type Dog implements Pet {
  name: String
  woofs: Boolean
}
```

pet.rs
``` rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlInterface, Debug, Clone)]
pub enum Pet {
    Cat(Cat),
    Dog(Dog),
}

#[GqlType(interface)]
impl Pet {
    async fn name(&self, ctx: &Context<'_>) -> Result<String, Error> {
        match self {
            Pet::Cat(obj) => obj.name(&ctx).await,
            Pet::Dog(obj) => obj.name(&ctx).await,
        }
    }
}

```

Each variants is possible types of interface.
