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

pub use rusty_gql_codegen::GqlModel;
pub use rusty_gql_codegen::GqlResolver;
