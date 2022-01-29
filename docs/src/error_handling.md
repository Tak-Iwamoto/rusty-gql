# Error Handling

If errors occur while GraphQL operation, `errors` field will be included in the response.

Add errors by using `add_error` of `Context`.

A error is defined by `GqlError` struct.

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
        Some(first) => {
          if first > 30 {
            // add error
            ctx.add_error(&GqlError::new("Up to 30 items at one time.", Some(ctx.item.position)));
            all_todos
          } else {
            all_todos.into_iter().take(first as usize).collect(),
          }
        }
        None => all_todos,
    }
}
```

When we want to add meta info, use `extensions`.

```rust
ctx.add_error(
    &GqlError::new("Error happens", Some(ctx.item.position)).set_extentions(
        GqlTypedError {
            error_type: GqlErrorType::Internal,
            error_detail: Some("Internal Error".to_string()),
            origin: None,
            debug_info: None,
            debug_uri: None,
        },
    ),
);
```

The GraphQL definition of rusty-gql error is as follows.
Also see [GraphQL spec](https://spec.graphql.org/June2018/#sec-Errors).

```graphql
type GqlError {
  message: String!
  locations: [Location!]!
  path: [String!]!
  extensions: GqlTypedError
}

type GqlTypedError {
  errorType: GqlErrorType!
  errorDetail: String
  origin: String
  debugInfo: DebugInfo
  debugUri: String
}

enum GqlErrorType {
  BadRequest
  FailedPreCondition
  Internal
  NotFound
  PermissionDenied
  Unauthenticated
  Unavailable
  Unknown
}
```
