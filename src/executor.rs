use crate::{
    container::ArcContainer,
    context::build_context,
    error::GqlError,
    operation::{build_operation, ArcOperation},
    request::Request,
    resolve_selection_parallelly, resolve_selection_serially,
    response::Response,
    validation::apply_validation,
    OperationType, SelectionSetResolver,
};

pub async fn execute<
    Query: SelectionSetResolver,
    Mutation: SelectionSetResolver,
    Subscription: SelectionSetResolver,
>(
    container: &ArcContainer<Query, Mutation, Subscription>,
    request: Request,
) -> Response {
    let query_doc = match graphql_parser::parse_query::<String>(&request.query) {
        Ok(doc) => doc,
        Err(_) => {
            let err = GqlError::new("failed to parse query", None);
            return Response::from_errors(vec![err]);
        }
    };
    let operation = build_operation(
        &query_doc,
        request.operation_name.clone(),
        request.variables.clone(),
    );

    let operation = match operation {
        Ok(op) => ArcOperation::new(op),
        Err(error) => return Response::from_errors(vec![error]),
    };

    if let Err(errors) = apply_validation(
        &container.schema,
        &query_doc,
        Some(&request.variables),
        &operation,
        request.operation_name.as_deref(),
    ) {
        return Response::from_errors(errors);
    }

    let ctx = build_context(&container.schema, &operation);

    let result = match operation.operation_type {
        OperationType::Query => {
            resolve_selection_parallelly(&ctx, &container.query_resolvers).await
        }
        OperationType::Mutation => {
            resolve_selection_serially(&ctx, &container.mutation_resolvers).await
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
            errors.extend(ctx.operation.errors.lock().unwrap().clone());
            Response::from_errors(errors)
        }
    }
}
