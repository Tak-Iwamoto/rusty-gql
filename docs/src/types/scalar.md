# Scalar

We can define custom scalars. The following is an example.

schema

```graphql
scalar Base64
```

scalar/base64.rs

```rust
#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlScalar)]
pub struct Base64(String);

impl GqlInputType for Base64 {
fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
    if let Some(GqlValue::String(v)) = value {
        let encoded = base64::encode(v);
        Ok(Base64(encoded))
    } else {
        Err(format!(
            "{}: is invalid type for UUID",
            value.unwrap_or(GqlValue::Null).to_string()
        ))
    }
}

fn into_gql_value(&self) -> GqlValue {
    GqlValue::String(self.0.to_string())
}
}
```

rusty-gql represents custom scalar by using `#[derive(GqlScalar)]` and `GqlInputType` trait.
