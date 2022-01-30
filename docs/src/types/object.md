# Object

rusty-gql defines GraphQL Object as Rust struct and `#[GqlType]` like the following.

src/graphql/resolver/todo.rs

```rust
#[derive(Clone)]
pub struct Todo {
    pub title: String,
    pub content: Option<String>,
    pub done: bool,
}

#[GqlType]
impl Todo {
    pub async fn title(&self, ctx: &Context<'_>) -> String {
        self.title.clone()
    }

    pub async fn content(&self, ctx: &Context<'_>) -> Option<String> {
        self.content.clone()
    }

    pub async fn done(&self, ctx: &Context<'_>) -> bool {
        self.done
    }
}
```

schema.graphql

```graphql
type Todo {
  title: String!
  content: String
  done: Boolean!
}
```

We'll implement `async fn` for each fields with `#[GqlType]`.

If we want to execute only when the field is included in a operation, implement `async fn` without the struct field.

src/graphql/resolver/todo.rs

```rust
#[derive(Clone)]
pub struct Todo {
    pub title: String,
    pub content: Option<String>,
    pub done: bool,
}

#[GqlType]
impl Todo {
    ...
    pub async fn user(&self, ctx: &Context<'_>) -> User {
      todo!()
    }
}
```

```graphql
type Todo {
  title: String!
  content: String
  done: Boolean!
  user: User!
}

type User {
  name
}
```
