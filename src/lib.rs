mod container;
mod context;
mod custom_directive;
mod error;
mod executor;
mod input;
mod operation;
mod path;
mod playground_html;
mod query_root;
mod request;
mod resolver;
mod response;
mod test_utils;
mod types;
mod validation;
mod variables;

use std::path::Path;

#[doc(hidden)]
pub use async_trait;

pub use container::Container;
pub use context::{ExecutionContext, FieldContext, SelectionSetContext};
pub use custom_directive::CustomDirective;
pub use error::{Error, GqlError};
pub use executor::execute;
use futures_util::Future;
pub use input::GqlInputType;
pub use operation::OperationType;
pub use playground_html::playground_html;
pub use query_root::QueryRoot;
pub use request::{receive_http_request, HttpRequestError, Request};
pub use resolver::{
    resolve_selection_parallelly, resolve_selection_serially, CollectFields, FieldResolver, Fields,
    SelectionSetResolver,
};
pub use response::Response;
pub use test_utils::{build_test_request, check_gql_response, schema_content};
pub use types::schema::build_schema;
pub use types::{
    DirectiveDefinition, EnumType, FieldType, GqlConstValue as Value, GqlDirective, GqlValue,
    GqlValueType, InputObjectType, InputValueType, InterfaceType, ObjectType, ScalarType, Schema,
    TypeDefinition, UnionType, ID,
};
pub use variables::Variables;

pub type ResolverResult<T> = ::std::result::Result<T, GqlError>;

pub use rusty_gql_macro::{GqlEnum, GqlInputObject, GqlInterface, GqlScalar, GqlType, GqlUnion};

#[derive(Clone)]
pub struct EmptyMutation;

#[async_trait::async_trait]
impl FieldResolver for EmptyMutation {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(None)
    }
    fn type_name() -> String {
        "Mutation".to_string()
    }
}

impl CollectFields for EmptyMutation {}

#[async_trait::async_trait]
impl SelectionSetResolver for EmptyMutation {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Null)
    }
}

#[derive(Clone)]
pub struct EmptySubscription;

#[async_trait::async_trait]
impl FieldResolver for EmptySubscription {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(None)
    }
    fn type_name() -> String {
        "Subscription".to_string()
    }
}

impl CollectFields for EmptySubscription {}

#[async_trait::async_trait]
impl SelectionSetResolver for EmptySubscription {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Null)
    }
}

pub type ResolveFut<'a> =
    &'a mut (dyn Future<Output = ResolverResult<Option<GqlValue>>> + Send + Unpin);

pub fn read_schemas(dir: &Path) -> std::io::Result<Vec<String>> {
    let mut schemas = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                read_schemas(&path)?;
            } else {
                let content = std::fs::read_to_string(path)?;
                schemas.push(content);
            }
        }
    }
    Ok(schemas)
}
