mod container;
mod context;
mod error;
mod executor;
mod graphiql_html;
mod input;
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
pub use error::{ErrorWrapper, GqlError};
pub use executor::execute;
pub use graphiql_html::playground_html;
pub use operation::OperationType;
pub use query_root::QueryRoot;
pub use request::{receive_http_request, HttpRequestError, Request};
pub use resolver::{FieldResolver, SelectionSetResolver};
pub use response::Response;
pub use types::schema::build_schema;
pub use types::{
    GqlArgument, GqlDirective, GqlDirectiveDefinition, GqlEnum, GqlField, GqlInputObject,
    GqlInterface, GqlObject, GqlScalar, GqlTypeDefinition, GqlUnion, GqlValue, Schema, ID,
};
pub use variables::Variables;

pub type ResolverResult<T> = ::std::result::Result<T, GqlError>;

pub use rusty_gql_codegen::Resolver;

#[derive(Clone)]
pub struct EmptyMutation;

#[async_trait::async_trait]
impl FieldResolver for EmptyMutation {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(None)
    }
}

#[derive(Clone)]
pub struct EmptySubscription;

#[async_trait::async_trait]
impl FieldResolver for EmptySubscription {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(None)
    }
}
