mod container;
mod context;
mod error;
mod executor;
mod operation;
mod path;
mod request;
mod resolver;
mod server;
mod template;
mod test_resolvers;
mod types;

#[doc(hidden)]
pub use async_trait;

pub use context::{ExecutionContext, FieldContext, SelectionSetContext};
use error::GqlError;
pub use operation::OperationType;
pub use resolver::{resolve_selection, Resolver, SelectionSetResolver};
pub use template::GraphiQLTemplate;
pub use types::schema::build_schema;
pub use types::{GqlField, GqlType, GqlValue, Schema};

pub type Response<T> = ::std::result::Result<T, GqlError>;

pub use rusty_gql_codegen::GqlModel;
pub use rusty_gql_codegen::Object;
