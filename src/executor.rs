use crate::{
    container::ArcContainer,
    context::build_context,
    error::GqlError,
    operation::{build_operation, ArcOperation},
    request::Request,
    response::Response,
    FieldResolver, OperationType,
};

pub async fn execute<Query: FieldResolver, Mutation: FieldResolver, Subscription: FieldResolver>(
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
    let operation = build_operation(&query_doc, request.operation_name, request.variables);

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
