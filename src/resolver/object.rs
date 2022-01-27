use std::collections::{BTreeMap, HashMap};

use serde::Serialize;

use crate::{
    types::value::serialize_into_gql_value, CollectFields, Context, FieldResolver, GqlValue,
    ResolverResult, SelectionSetContext, SelectionSetResolver,
};

#[async_trait::async_trait]
impl<K, V> FieldResolver for BTreeMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
    async fn resolve_field(&self, _ctx: &Context<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut map = BTreeMap::new();
        for (name, v) in self {
            map.insert(
                name.to_string(),
                serialize_into_gql_value(v).unwrap_or_default(),
            );
        }
        Ok(Some(GqlValue::Object(map)))
    }
    fn type_name() -> String {
        "Object".to_string()
    }
}

impl<K, V> CollectFields for BTreeMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
}

#[async_trait::async_trait]
impl<K, V> SelectionSetResolver for BTreeMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut map = BTreeMap::new();
        for (name, v) in self {
            map.insert(
                name.to_string(),
                serialize_into_gql_value(v).unwrap_or_default(),
            );
        }
        Ok(GqlValue::Object(map))
    }
}

#[async_trait::async_trait]
impl<K, V> FieldResolver for HashMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
    async fn resolve_field(&self, _ctx: &Context<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut map = BTreeMap::new();
        for (name, v) in self {
            map.insert(
                name.to_string(),
                serialize_into_gql_value(v).unwrap_or_default(),
            );
        }
        Ok(Some(GqlValue::Object(map)))
    }
    fn type_name() -> String {
        "Object".to_string()
    }
}

impl<K, V> CollectFields for HashMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
}

#[async_trait::async_trait]
impl<K, V> SelectionSetResolver for HashMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut map = BTreeMap::new();
        for (name, v) in self {
            map.insert(
                name.to_string(),
                serialize_into_gql_value(v).unwrap_or_default(),
            );
        }
        Ok(GqlValue::Object(map))
    }
}
