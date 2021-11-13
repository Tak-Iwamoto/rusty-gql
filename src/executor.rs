use crate::{
    context::build_context,
    operation::{build_operation, ArcOperation},
    request::Request,
    types::schema::ArcSchema,
    OperationType,
};

pub async fn execute(schema: &ArcSchema, request: Request) -> Result<(), String> {
    let operation = build_operation(&request.query_doc, &schema, request.operation_name)?;
    let operation = ArcOperation::new(operation);
    let ctx = build_context(schema, &operation);

    match operation.definition.operation_type {
        OperationType::Query => {
            println!("{:?}", "query");
            println!("{:?}", ctx.operation);
        }
        OperationType::Mutation => {
            println!("{:?}", "mutation");
            println!("{:?}", ctx.operation);
        }
        OperationType::Subscription => {
            println!("{:?}", "subscription");
            println!("{:?}", ctx);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{build_schema, request::Request, types::schema::ArcSchema};

    use super::execute;

    #[tokio::test]
    async fn it_works() {
        let schema_doc = fs::read_to_string("src/tests/github.graphql").unwrap();
        let schema = ArcSchema::new(build_schema(schema_doc.as_str()).unwrap());
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let request = Request {
            query_doc,
            operation_name: None,
        };

        execute(&schema, request).await;
    }
}
