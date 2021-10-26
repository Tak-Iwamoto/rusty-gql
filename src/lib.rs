mod container;
mod error;
mod executor;
mod graphql_object;
mod graphql_value;
mod operation;
mod request;
mod resolver;
mod server;
mod template;
mod types;

use error::GraphQLError;
pub use resolver::Resolver;
pub use template::GraphiQLTemplate;
pub use types::GraphQLSchema;

pub type GraphQLResponse<T> = ::std::result::Result<T, GraphQLError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
