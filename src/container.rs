use std::{ops::Deref, sync::Arc};

use crate::{
    error::GqlError,
    types::schema::{build_schema, ArcSchema},
    FieldResolver,
};

pub struct Container<Query: FieldResolver, Mutation: FieldResolver, Subscription: FieldResolver> {
    pub query_resolvers: Query,
    pub mutation_resolvers: Mutation,
    pub subscription_resolvers: Subscription,
    pub schema: ArcSchema,
}

#[derive(Clone)]
pub struct ArcContainer<Query: FieldResolver, Mutation: FieldResolver, Subscription: FieldResolver>(
    Arc<Container<Query, Mutation, Subscription>>,
);

impl<Query, Mutation, Subscription> Deref for ArcContainer<Query, Mutation, Subscription>
where
    Query: FieldResolver,
    Mutation: FieldResolver,
    Subscription: FieldResolver,
{
    type Target = Container<Query, Mutation, Subscription>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Query, Mutation, Subscription> ArcContainer<Query, Mutation, Subscription>
where
    Query: FieldResolver,
    Mutation: FieldResolver,
    Subscription: FieldResolver,
{
    pub fn new(
        schema_doc: &[&str],
        query: Query,
        mutation: Mutation,
        subscription: Subscription,
    ) -> Result<Self, GqlError> {
        let schema = build_schema(schema_doc)?;
        Ok(ArcContainer(Arc::new(Container {
            query_resolvers: query,
            mutation_resolvers: mutation,
            subscription_resolvers: subscription,
            schema: ArcSchema::new(schema),
        })))
    }
}
