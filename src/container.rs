use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Deref,
    sync::Arc,
};

use crate::{
    context::ExecutionContext,
    field_resolver::FieldResolver,
    operation::{build_operation, ArcOperation, Operation},
    path::GraphQLPath,
    request::Request,
    types::schema::{build_schema, ArcSchema},
};

pub struct ContextData(HashMap<TypeId, Box<dyn Any + Sync + Send>>);

impl Default for ContextData {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct Container<
    'a,
    Query: FieldResolver,
    Mutation: FieldResolver,
    Subscription: FieldResolver,
> {
    query_resolvers: Query,
    mutation_resolvers: Mutation,
    subscription_resolvers: Subscription,
    schema: ArcSchema<'a>,
    context_data: ContextData,
}

pub struct ArcContainer<
    'a,
    Query: FieldResolver,
    Mutation: FieldResolver,
    Subscription: FieldResolver,
>(Arc<Container<'a, Query, Mutation, Subscription>>);

impl<'a, Query: FieldResolver, Mutation: FieldResolver, Subscription: FieldResolver> Deref
    for ArcContainer<'a, Query, Mutation, Subscription>
{
    type Target = Container<'a, Query, Mutation, Subscription>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, Query, Mutation, Subscription> ArcContainer<'a, Query, Mutation, Subscription>
where
    Query: FieldResolver,
    Mutation: FieldResolver,
    Subscription: FieldResolver,
{
    pub fn new(
        schema_doc: &'a str,
        query: Query,
        mutation: Mutation,
        subscription: Subscription,
    ) -> Result<Self, String> {
        let schema = build_schema(schema_doc)?;
        Ok(ArcContainer(Arc::new(Container {
            query_resolvers: query,
            mutation_resolvers: mutation,
            subscription_resolvers: subscription,
            schema: ArcSchema::new(schema),
            context_data: ContextData::default(),
        })))
    }

    async fn prepare_operation(&'a self, request: &'a Request) -> Result<Operation<'a>, String> {
        let query_doc = &request.query_doc;
        let operation_name = request.operation_name.clone();
        let operation = build_operation(query_doc.as_str(), &self.schema, operation_name)?;
        Ok(operation)
    }

    // async fn execute_operation(&'a self, operation: ArcOperation<'a>) {
    //     let operation_type = operation.definition.operation_type.to_string();
    //     let root_fieldname = operation.definition.root_field.name.to_string();
    //     let selection_set = &operation.definition.selection_set;
    //     let current_field = operation.definition.root_field.clone();
    //     let current_path = GraphQLPath::default()
    //         .prev(None)
    //         .key(root_fieldname)
    //         .parent_name(operation_type);

    //     let ctx = ExecutionContext {
    //         schema: &self.schema,
    //         operation: &operation.clone(),
    //         current_field,
    //         current_path,
    //     };

    //     match &ctx.operation.definition.operation_type {
    //         crate::operation::OperationType::Query => {

    //         },
    //         crate::operation::OperationType::Mutation => todo!(),
    //         crate::operation::OperationType::Subscription => todo!(),
    //     }
    // }

    // pub async fn execute(&'a self, request: &'a Request) {
    //     match self.prepare_operation(request).await {
    //         Ok(operation) => {
    //             async move {
    //                 self.execute_operation(ArcOperation::new(operation));
    //             };
    //         }
    //         Err(error) => todo!(),
    //     }
    // }
}
