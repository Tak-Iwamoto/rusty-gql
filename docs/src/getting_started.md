# Getting Started

## Install rusty-gql-cli

```
cargo install rusty-gql-cli
```

## Run new command

```
rusty-gql new gql-example
cd gql-example
```

## Start the GraphQL Server

```
cargo run
```

## Creating a GraphQL Schema

rusty-gql is designed for schema first development.
It reads any graphql files under `schema/**`.

`schema/schema.graphql`

```graphql
type Query {
  todos(first: Int): [Todo!]!
}

type Todo {
  title: String!
  content: String
  done: Boolean!
}
```

## Implement Resolvers

Let's edit `src/graphql/query/todos.rs`.

```rust
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

## Generate Rust code

Edit schema.graphql.

```graphql
type Query {
  todos(first: Int): [Todo!]!
  # added
  todo(id: ID!): Todo
}

type Todo {
  title: String!
  description: String
  done: Boolean!
}
```

rusty-gql generates rust code from graphql schema files.

```
rusty-gql generate // or rusty-gql g
```

### Directory Structure

```
src
 ┣ graphql
 ┃ ┣ directive
 ┃ ┃ ┗ mod.rs
 ┃ ┣ input
 ┃ ┃ ┗ mod.rs
 ┃ ┣ mutation
 ┃ ┃ ┗ mod.rs
 ┃ ┣ query
 ┃ ┃ ┣ mod.rs
 ┃ ┃ ┣ todo.rs
 ┃ ┃ ┗ todos.rs
 ┃ ┣ resolver
 ┃ ┃ ┣ mod.rs
 ┃ ┃ ┗ todo.rs
 ┃ ┣ scalar
 ┃ ┃ ┗ mod.rs
 ┃ ┣ subscription
 ┃ ┃ ┗ mod.rs
 ┃ ┗ mod.rs
 ┗ main.rs
```

## GraphQL Playground

rusty-gql supports GraphiQL playground.
Open a browser to http://localhost:3000/graphiql.
