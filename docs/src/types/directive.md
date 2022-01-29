# Directive

rusty-gql uses directives as middleware.

It is useful in the following use cases.

- Authorization
- Validation
- Caching
- Logging, metrics
- etc.

If we don't want to expose a specific field, we can define the following directive.

schema.graphql

```graphql
type User {
  name: String!
  password_hash: String @hidden
}
directive @hidden on FIELD_DEFINITION | OBJECT
```

directive/hidden.rs

```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(Clone)]
struct hidden;

#[async_trait::async_trait]
impl CustomDirective for hidden {
    async fn resolve_field(
        &self,
        _ctx: &Context<'_>,
        _directive_args: &BTreeMap<String, GqlValue>,
        resolve_fut: ResolveFut<'_>,
    ) -> ResolverResult<Option<GqlValue>> {
      resolve_fut.await.map(|_v| None)
    }
}
```
