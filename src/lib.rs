mod container;
mod context;
mod error;
mod field_resolver;
mod graphql_object;
mod graphql_value;
mod operation;
mod path;
mod request;
mod resolver;
mod server;
mod template;
mod test_resolvers;
mod types;

use error::GraphQLError;
pub use operation::OperationType;
pub use resolver::Resolver;
pub use template::GraphiQLTemplate;
pub use types::schema::build_schema;
pub use types::{GqlField, GqlType, Schema};

pub type GraphQLResponse<T> = ::std::result::Result<T, GraphQLError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
