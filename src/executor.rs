use crate::{
    container::ArcContainer,
    context::build_context,
    error::GqlError,
    operation::{build_operation, ArcOperation},
    request::Request,
    response::Response,
    OperationType, Resolver,
};

pub async fn execute<Query: Resolver, Mutation: Resolver, Subscription: Resolver>(
    container: &ArcContainer<Query, Mutation, Subscription>,
    request: Request,
) -> Response {
    let operation = build_operation(&request.query, &container.schema, request.operation_name);

    let operation = match operation {
        Ok(op) => ArcOperation::new(op),
        Err(error) => return Response::from_errors(vec![error]),
    };

    let ctx = build_context(&container.schema, &operation);

    let ctx_selection_set = &ctx.with_selection_set(&operation.selection_set);

    let result = match operation.operation_type {
        OperationType::Query => {
            ctx_selection_set
                .resolve_selection_parallelly(&container.query_resolvers)
                .await
        }
        OperationType::Mutation => {
            ctx_selection_set
                .resolve_selection_serially(&container.mutation_resolvers)
                .await
        }
        OperationType::Subscription => {
            let error = GqlError::new("subscription cannot execute from this path", None);
            return Response::from_errors(vec![error]);
        }
    };

    match result {
        Ok(value) => Response::new(value),
        Err(error) => {
            let mut errors = vec![error];
            errors.extend(ctx_selection_set.operation.errors.lock().unwrap().clone());
            Response::from_errors(errors)
        }
    }
}
