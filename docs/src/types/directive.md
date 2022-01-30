# Directive

We can use directives as middleware.

It is useful in the following use cases.

- Authorization
- Validation
- Caching
- Logging, metrics
- etc.

If we don't want to expose a specific field, we can define the following directive.

src/graphql/directive/hidden.rs

```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(Clone)]
struct Hidden;

impl Hidden {
    fn new() -> Box<dyn CustomDirective> {
        Box::new(Hidden {})
    }
}

#[async_trait::async_trait]
impl CustomDirective for Hidden {
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

schema.graphql

```graphql
type User {
  name: String!
  password_hash: String @hidden
}
directive @hidden on FIELD_DEFINITION | OBJECT
```

Need to pass a HashMap of directives when Container::new in main.rs.
key is the directive name, value is the directive struct.

main.rs

```rust
async fn main() {
    ...
    let mut custom_directive_maps = HashMap::new();
    custom_directive_maps.insert("hidden", Hidden::new());

    let container = Container::new(
        schema_docs.as_slice(),
        Query,
        Mutation,
        EmptySubscription,
        custom_directive_maps, // path here
    )
    .unwrap();
    ...
}
```
