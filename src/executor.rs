use crate::{
    container::ArcContainer,
    context::build_context,
    operation::{build_operation, ArcOperation},
    request::Request,
    resolver::{resolve_mutation, resolve_query, resolve_subscription},
    OperationType, Resolver,
};

pub async fn execute<T: Resolver>(
    container: &ArcContainer<T>,
    request: Request,
) -> Result<(), String> {
    let operation = build_operation(
        &request.query_doc,
        &container.schema,
        request.operation_name,
    )?;
    let operation = ArcOperation::new(operation);
    let ctx = build_context(&container.schema, &operation);

    let result = match operation.operation_type {
        OperationType::Query => resolve_query(&ctx, &container.query_resolvers).await,
        OperationType::Mutation => resolve_mutation(&ctx, &container.mutation_resolvers).await,
        OperationType::Subscription => {
            resolve_subscription(&ctx, &container.subscription_resolvers).await
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{build_schema, request::Request, types::schema::ArcSchema};

    #[tokio::test]
    async fn it_works() {
        let schema_doc = fs::read_to_string("src/tests/github.graphql").unwrap();
        let schema = ArcSchema::new(build_schema(schema_doc.as_str()).unwrap());
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let request = Request {
            query_doc,
            operation_name: None,
        };
    }
}
