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

Query resolvers.

[Query](./schema/query.md)

## src/graphql/mutation

Mutation resolvers.

[Mutation](./schema/mutation.md)

## src/graphql/resolver

GraphQL `Object`, `Enum`, `Union`, `Interface` types.

- [Object](./types/object.md)
- [Enum](./types/enum.md)
- [Union](./types/union.md)
- [Interface](./types/interface.md)

## src/graphql/input

GraphQL InputObject.

[InputObject](./types/input_object.md)

## src/graphql/scalar

Custom scalars.

[Scalar](./types/scalar.md)

## src/graphql/directive

Custom directives.

[Directive](./types/directive.md)
