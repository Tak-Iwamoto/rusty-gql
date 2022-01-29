# Query

rusty-gql has `Query` files under `src/graphql/query/**`.

For example,
```
src
 ┣ graphql
 ┃ ┣ query
 ┃ ┃ ┣ mod.rs
 ┃ ┃ ┗ todos.rs
 ```

src/graphql/query/todos.rs
```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

pub async fn todos(ctx: &Context<'_>, first: Option<i32>) -> Vec<Todo> {
    let all_todos = vec![
        Todo {
            title: "Programming".to_string(),
            content: Some("Learn Rust".to_string()),
            done: false,
        },
        Todo {
            title: "Shopping".to_string(),
            content: None,
            done: true,
        },
    ];
    match first {
        Some(first) => all_todos.into_iter().take(first as usize).collect(),
        None => all_todos,
    }
}
```

src/graphql/query/mod.rs
```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;
mod todos;

#[derive(Clone)]
pub struct Query;

#[GqlType]
impl Query {
    pub async fn todos(&self, ctx: &Context<'_>, first: Option<i32>) -> Vec<Todo> {
        todos::todos(&ctx,first).await
    }
}
```

Files except for `mod.rs` implements resolvers for each Query Type fields.

`mod.rs` only bundles these files and defines `Query` struct.
