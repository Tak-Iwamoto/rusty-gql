use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use crate::{
    context::build_context,
    object_resolver::ObjectResolver,
    operation::{build_operation, Operation},
    request::Request,
    types::schema::build_schema,
    Schema,
};

pub struct ContextData(HashMap<TypeId, Box<dyn Any + Sync + Send>>);

impl Default for ContextData {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct Container<
    'a,
    Query: ObjectResolver,
    Mutation: ObjectResolver,
    Subscription: ObjectResolver,
> {
    query_resolvers: Query,
    mutation_resolvers: Mutation,
    subscription_resolvers: Subscription,
    schema: Schema<'a>,
    context_data: ContextData,
}

pub struct ArcContainer<
    'a,
    Query: ObjectResolver,
    Mutation: ObjectResolver,
    Subscription: ObjectResolver,
>(Arc<Container<'a, Query, Mutation, Subscription>>);

impl<'a, Query, Mutation, Subscription> Container<'a, Query, Mutation, Subscription>
where
    Query: ObjectResolver,
    Mutation: ObjectResolver,
    Subscription: ObjectResolver,
{
    pub fn new(
        schema_doc: &'a str,
        query: Query,
        mutation: Mutation,
        subscription: Subscription,
    ) -> Result<Self, String> {
        let schema = build_schema(schema_doc)?;
        Ok(Container {
            query_resolvers: query,
            mutation_resolvers: mutation,
            subscription_resolvers: subscription,
            schema,
            context_data: ContextData::default(),
        })
    }

    async fn prepare_operation(&'a self, request: &'a Request) -> Result<Operation<'a>, String> {
        let query_doc = &request.query_doc;
        let operation_name = request.operation_name.clone();
        let operation = build_operation(query_doc.as_str(), &self.schema, operation_name)?;
        Ok(operation)
    }

    // async fn execute_operation(&'a self, operation: &'a Operation<'a>) {
    //     let context = build_context(&self.schema, &operation.clone());
    // }

    // pub async fn execute(&'a self, request: &'a Request) {
    //     let request = request.clone();
    //     match self.prepare_operation(&request).await {
    //         Ok(operation) => {
    //             self.execute_operation(&operation.clone());
    //         }
    //         Err(error) => todo!(),
    //     }
    // }
}
