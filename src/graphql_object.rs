use crate::{graphql_value::GraphQLValue, GraphQLResponse};
use async_trait::async_trait;
use graphql_parser::schema::Field;

#[async_trait]
pub trait FieldResolveer {
    async fn resolve_field(&self) -> GraphQLResponse<GraphQLValue>;
}

pub struct GraphQLObject<'a> {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field<'a, &'a str>>,
}
