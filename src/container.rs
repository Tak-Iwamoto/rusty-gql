use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use crate::object_resolver::ObjectResolver;

pub struct ContextData(HashMap<TypeId, Box<dyn Any + Sync + Send>>);

pub struct Container<Query: ObjectResolver, Mutation: ObjectResolver, Subscription: ObjectResolver>
{
    query_resolvers: Query,
    mutation_resolvers: Mutation,
    subscription_resolvers: Subscription,
    context_data: ContextData,
}

pub struct ArcContainer<
    Query: ObjectResolver,
    Mutation: ObjectResolver,
    Subscription: ObjectResolver,
>(Arc<Container<Query, Mutation, Subscription>>);
