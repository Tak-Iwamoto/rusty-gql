# Directive

rusty-gql uses directives as middleware.

It is useful in the following use cases.

- Authorization
- Validation
- Caching
- Logging, metrics
- etc.

schema.graphql

```graphql
directive @auth(requires: Role!) on FIELD_DEFINITION | OBJECT

enum Role {
  ADMIN
  USER
}
```

directive/auth.rs

```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(Clone)]
struct auth;

#[async_trait::async_trait]
impl CustomDirective for auth {
    async fn resolve_field(
        &self,
        _ctx: &Context<'_>,
        directive_args: &BTreeMap<String, GqlValue>,
        resolve_fut: ResolveFut<'_>,
    ) -> ResolverResult<Option<GqlValue>> {
      todo!()
    }
}
```
