use std::collections::BTreeMap;

use serde::Serialize;

use crate::{
    FieldContext, GqlValue, Resolver, ResolverResult, SelectionSetContext, SelectionSetResolver,
};

// #[async_trait::async_trait]
// impl<K, V> Resolver for BTreeMap<K, V>
// where
//     K: ToString + Eq + Send + Sync,
//     V: Serialize + Send + Sync,
// {
//     async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
//         let mut map = BTreeMap::new();
//         for (name, v) in self {
//             map.insert(name.to_string(), v);
//         }
//         Ok(Some(GqlValue::Object(map)))
//     }
// }
