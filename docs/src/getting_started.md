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
It reads any graphql files under `/schema`

For example,

`schema/schema.graphql`
``` graphql
type Query {
  todos(first: Int): [Todo!]!
}

type Todo {
  title: String!
  description: String
  done: Boolean!
}
```

## Generate Rust code
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
 ┃ ┃ ┗ todos.rs
 ┃ ┣ resolver
 ┃ ┃ ┗ todo.rs
 ┃ ┣ scalar
 ┃ ┃ ┗ mod.rs
 ┃ ┣ subscription
 ┃ ┃ ┗ mod.rs
 ┃ ┗ mod.rs
 ┗ main.rs
```

## Implement Resolvers
## GraphQL Playground
rusty-gql supports GraphiQL playground.
Open a browser to http://localhost:3000/graphiql.
