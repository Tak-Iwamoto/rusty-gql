mod boolean;
mod number;
mod object;
mod string;

use async_trait::async_trait;
use futures_util::future::BoxFuture;
use graphql_parser::{query::Field, schema::Type};

use crate::{
    context::{FieldContext, SelectionSetContext},
    GqlTypeDefinition, GqlValue, ResolverResult, Schema,
};

pub type ResolverFuture<'a> = BoxFuture<'a, ResolverResult<(String, GqlValue)>>;

#[async_trait]
pub trait SelectionSetResolver: Resolver {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue>;
}

#[async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>>;
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for &T {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        T::resolve_field(*self, ctx).await
    }
}
