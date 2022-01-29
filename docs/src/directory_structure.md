# Directory Structure

A rusty-gql project has the following directory structure.

```
rusty-gql-project
 ┣ schema
 ┃ ┗ schema.graphql
 ┣ src
 ┃ ┣ graphql
 ┃ ┃ ┣ directive
 ┃ ┃ ┃ ┗ mod.rs
 ┃ ┃ ┣ input
 ┃ ┃ ┃ ┗ mod.rs
 ┃ ┃ ┣ mutation
 ┃ ┃ ┃ ┗ mod.rs
 ┃ ┃ ┣ query
 ┃ ┃ ┃ ┣ mod.rs
 ┃ ┃ ┣ resolver
 ┃ ┃ ┃ ┣ mod.rs
 ┃ ┃ ┣ scalar
 ┃ ┃ ┃ ┗ mod.rs
 ┃ ┃ ┣ subscription
 ┃ ┃ ┃ ┗ mod.rs
 ┃ ┃ ┗ mod.rs
 ┃ ┗ main.rs
 ┗ Cargo.toml
```

## schema

GraphQL schema files are located under `schema/**`.

We can also place multiple GraphQL files.

For example, like this.

```
schema
 ┣ post
 ┃ ┗ post.graphql
 ┣ user
 ┃ ┗ user.graphql
 ┗ index.graphql
```

## src/graphql/query

Query resolvers are placed under `src/graphql/query/`.

[Query](./schema/query.md)

## src/graphql/mutation

Mutation resolvers are placed under `src/graphql/mutation/`.

[Mutation](./schema/mutation.md)

## src/graphql/resolver

GraphQL `Object`, `Enum`, `Union`, `Interface` types are located under `src/graphql/resolver`.

- [Object](./types/object.md)
- [Enum](./types/enum.md)
- [Union](./types/union.md)
- [Interface](./types/interface.md)

## src/graphql/scalar

Custom scalars are located under `src/graphql/scalar`.

[Scalar](./types/scalar.md)

## src/graphql/directive

We can define custom directives in `src/graphql/directive`.

[Directive](./types/directive.md)
