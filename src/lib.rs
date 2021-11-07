mod object_resolver;
mod error;
mod context;
mod graphql_object;
mod graphql_value;
mod operation;
mod resolver;
mod container;
mod server;
mod template;
mod types;
mod request;
mod path;
mod test_resolver;
mod test_resolvers;

use error::GraphQLError;
pub use resolver::Resolver;
pub use template::GraphiQLTemplate;
pub use types::Schema;

pub type GraphQLResponse<T> = ::std::result::Result<T, GraphQLError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
