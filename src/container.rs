use std::{ops::Deref, sync::Arc};

use crate::{
    error::GqlError,
    types::schema::{build_schema, Schema},
    SelectionSetResolver,
};

pub struct ContainerInner<
    Query: SelectionSetResolver,
    Mutation: SelectionSetResolver,
    Subscription: SelectionSetResolver,
> {
    pub query_resolvers: Query,
    pub mutation_resolvers: Mutation,
    pub subscription_resolvers: Subscription,
    pub schema: Schema,
}

#[derive(Clone)]
pub struct Container<
    Query: SelectionSetResolver,
    Mutation: SelectionSetResolver,
    Subscription: SelectionSetResolver,
>(Arc<ContainerInner<Query, Mutation, Subscription>>);

impl<Query, Mutation, Subscription> Deref for Container<Query, Mutation, Subscription>
where
    Query: SelectionSetResolver,
    Mutation: SelectionSetResolver,
    Subscription: SelectionSetResolver,
{
    type Target = ContainerInner<Query, Mutation, Subscription>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Query, Mutation, Subscription> Container<Query, Mutation, Subscription>
where
    Query: SelectionSetResolver,
    Mutation: SelectionSetResolver,
    Subscription: SelectionSetResolver,
{
    pub fn new(
        schema_doc: &[&str],
        query: Query,
        mutation: Mutation,
        subscription: Subscription,
    ) -> Result<Self, GqlError> {
        let schema = build_schema(schema_doc)?;
        Ok(Container(Arc::new(ContainerInner {
            query_resolvers: query,
            mutation_resolvers: mutation,
            subscription_resolvers: subscription,
            schema,
        })))
    }
}
