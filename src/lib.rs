mod container;
mod context;
mod error;
mod executor;
mod graphiql_html;
mod operation;
mod path;
mod query_root;
mod request;
mod resolver;
mod response;
mod types;
mod validation;
mod variables;

#[doc(hidden)]
pub use async_trait;

pub use container::ArcContainer;
pub use context::{ExecutionContext, FieldContext, SelectionSetContext};
use error::GqlError;
pub use executor::execute;
pub use graphiql_html::playground_html;
pub use operation::OperationType;
pub use request::{receive_http_request, HttpRequestError, Request};
pub use resolver::{Resolver, SelectionSetResolver};
pub use response::Response;
pub use types::schema::build_schema;
pub use types::{
    GqlArgument, GqlDirective, GqlEnum, GqlField, GqlInputObject, GqlInterface, GqlObject,
    GqlScalar, GqlTypeDefinition, GqlUnion, GqlValue, Schema,
};

pub type ResolverResult<T> = ::std::result::Result<T, GqlError>;

pub use rusty_gql_codegen::GqlResolver;
pub use rusty_gql_codegen::GqlData;

#[derive(Clone)]
pub struct EmptyMutation;

#[async_trait::async_trait]
impl Resolver for EmptyMutation {
    async fn resolve_field(
        &self,
        ctx: &FieldContext<'_>,
    ) -> ResolverResult<Option<GqlValue>> {
        Ok(None)
    }
}

#[derive(Clone)]
pub struct EmptySubscription;

#[async_trait::async_trait]
impl Resolver for EmptySubscription {
    async fn resolve_field(
        &self,
        ctx: &FieldContext<'_>,
    ) -> ResolverResult<Option<GqlValue>> {
        Ok(None)
    }
}
