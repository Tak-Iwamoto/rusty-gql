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
