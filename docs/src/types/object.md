# Object

rusty-gql defines GraphQL Object as Rust struct and `#[GqlType]` like the following.

schema.graphql

```graphql
type Todo {
  title: String!
  content: String
  done: Boolean!
}
```

todo.rs

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

We'll implement async fn for each fields with `#[GqlType]`.

If we want to execute the field resolver only when the query includes the field, we can define only async fn without the struct field.

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

todo.rs

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
