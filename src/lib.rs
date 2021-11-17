mod container;
mod context;
mod error;
mod executor;
mod field_resolver;
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

use error::GqlError;
pub use context::ExecutionContext;
pub use operation::OperationType;
pub use resolver::Resolver;
pub use field_resolver::FieldResolver;
pub use template::GraphiQLTemplate;
pub use types::schema::build_schema;
pub use types::{GqlField, GqlType, GqlValue, Schema};

pub type Response<T> = ::std::result::Result<T, GqlError>;

pub use rusty_gql_codegen::gql_object;
