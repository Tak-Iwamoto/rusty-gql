use std::collections::{BTreeSet, HashSet, LinkedList, VecDeque};

use crate::{
    FieldContext, GqlValue, Resolver, ResolverResult, SelectionSetContext, SelectionSetResolver,
};

#[async_trait::async_trait]
impl<T: Resolver, const N: usize> Resolver for [T; N] {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut result = Vec::new();
        for value in self {
            if let Some(v) = value.resolve_field(ctx).await? {
                result.push(v);
            }
        }
        Ok(Some(GqlValue::List(result)))
    }
}

#[async_trait::async_trait]
impl<T: SelectionSetResolver, const N: usize> SelectionSetResolver for [T; N] {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut result = Vec::new();
        for value in self {
            let v = value.resolve_selection_set(ctx).await?;
            result.push(v);
        }
        Ok(GqlValue::List(result))
    }
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for HashSet<T> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut result = Vec::new();
        for value in self {
            if let Some(v) = value.resolve_field(ctx).await? {
                result.push(v);
            }
        }
        Ok(Some(GqlValue::List(result)))
    }
}

#[async_trait::async_trait]
impl<T: SelectionSetResolver> SelectionSetResolver for HashSet<T> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut result = Vec::new();
        for value in self {
            let v = value.resolve_selection_set(ctx).await?;
            result.push(v);
        }
        Ok(GqlValue::List(result))
    }
}

#[async_trait::async_trait]
impl<'a, T: Resolver + 'a> Resolver for &'a [T] {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut result = Vec::new();
        for value in self.iter() {
            if let Some(v) = value.resolve_field(ctx).await? {
                result.push(v);
            }
        }
        Ok(Some(GqlValue::List(result)))
    }
}

#[async_trait::async_trait]
impl<'a, T: SelectionSetResolver + 'a> SelectionSetResolver for &'a [T] {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut result = Vec::new();
        for value in self.iter() {
            let v = value.resolve_selection_set(ctx).await?;
            result.push(v);
        }
        Ok(GqlValue::List(result))
    }
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for VecDeque<T> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut result = Vec::new();
        for value in self.iter() {
            if let Some(v) = value.resolve_field(ctx).await? {
                result.push(v);
            }
        }
        Ok(Some(GqlValue::List(result)))
    }
}

#[async_trait::async_trait]
impl<T: SelectionSetResolver> SelectionSetResolver for VecDeque<T> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut result = Vec::new();
        for value in self.iter() {
            let v = value.resolve_selection_set(ctx).await?;
            result.push(v);
        }
        Ok(GqlValue::List(result))
    }
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for LinkedList<T> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut result = Vec::new();
        for value in self.iter() {
            if let Some(v) = value.resolve_field(ctx).await? {
                result.push(v);
            }
        }
        Ok(Some(GqlValue::List(result)))
    }
}

#[async_trait::async_trait]
impl<T: SelectionSetResolver> SelectionSetResolver for LinkedList<T> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut result = Vec::new();
        for value in self.iter() {
            let v = value.resolve_selection_set(ctx).await?;
            result.push(v);
        }
        Ok(GqlValue::List(result))
    }
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for Vec<T> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut result = Vec::new();
        for value in self.iter() {
            if let Some(v) = value.resolve_field(ctx).await? {
                result.push(v);
            }
        }
        Ok(Some(GqlValue::List(result)))
    }
}

#[async_trait::async_trait]
impl<T: SelectionSetResolver> SelectionSetResolver for Vec<T> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut result = Vec::new();
        for value in self.iter() {
            let v = value.resolve_selection_set(ctx).await?;
            result.push(v);
        }
        Ok(GqlValue::List(result))
    }
}
