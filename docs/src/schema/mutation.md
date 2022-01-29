# Mutation

Mutation has a similar directory structure to Query.

rusty-gql has Mutation files under `src/graphql/mutation/**`.

```
src
 ┣ graphql
 ┃ ┣ mutation
 ┃ ┃ ┣ mod.rs
 ┃ ┃ ┗ create_todo.rs
```

src/graphql/mutation/create_todo.rs

```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

pub async fn createTodo(ctx: &Context<'_>, input: TodoInput) -> Todo {
  ...
}
```

src/graphql/query/mod.rs

```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;
mod create_todo;

#[derive(Clone)]
pub struct Mutation;

#[GqlType]
impl Mutation {
    pub async fn todos(&self, ctx: &Context<'_>, input: TodoInput) -> Todo {
        create_todo::createTodo(ctx, input).await
    }
}
```

Mutation is optional, so if we don't need Mutation, use `EmptyMutation` struct in `main.rs`

main.rs

```rust
mod graphql;
...

#[tokio::main]
async fn main() {
    ...
    let container = Container::new(
        schema_docs.as_slice(),
        Query,
        EmptyMutation, // or graphql::Mutation
        EmptySubscription,
        Default::default(),
    )
    .unwrap();
}

```
