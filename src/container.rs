use std::{ops::Deref, sync::Arc};

use crate::{
    error::GqlError,
    types::schema::{build_schema, ArcSchema},
    Resolver,
};

pub struct Container<T: Resolver> {
    pub query_resolvers: T,
    pub mutation_resolvers: T,
    pub subscription_resolvers: T,
    pub schema: ArcSchema,
}

pub struct ArcContainer<T: Resolver>(Arc<Container<T>>);

impl<T> Deref for ArcContainer<T>
where
    T: Resolver,
{
    type Target = Container<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ArcContainer<T>
where
    T: Resolver,
{
    pub fn new(schema_doc: &str, query: T, mutation: T, subscription: T) -> Result<Self, GqlError> {
        let schema = build_schema(schema_doc)?;
        Ok(ArcContainer(Arc::new(Container {
            query_resolvers: query,
            mutation_resolvers: mutation,
            subscription_resolvers: subscription,
            schema: ArcSchema::new(schema),
        })))
    }
}
