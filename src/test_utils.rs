use crate::{execute, ArcContainer, Request, SelectionSetResolver, Variables};

pub fn schema_content(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub fn build_test_request(
    query: &str,
    operation_name: Option<String>,
    variables: Variables,
) -> Request {
    Request {
        query: query.to_string(),
        operation_name,
        variables,
    }
}

pub async fn check_gql_response<
    Query: SelectionSetResolver,
    Mutation: SelectionSetResolver,
    Subscription: SelectionSetResolver,
>(
    request: Request,
    expected_response: &str,
    container: &ArcContainer<Query, Mutation, Subscription>,
) {
    let res = execute(&container, request).await;
    assert_eq!(serde_json::to_string(&res).unwrap(), expected_response);
}
