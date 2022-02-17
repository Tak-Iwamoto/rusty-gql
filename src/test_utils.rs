use crate::{execute, Container, Request, SelectionSetResolver, Variables};

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
        extensions: Default::default(),
    }
}

pub async fn check_gql_response<
    Query: SelectionSetResolver + 'static,
    Mutation: SelectionSetResolver + 'static,
    Subscription: SelectionSetResolver + 'static,
>(
    request: Request,
    expected_response: &str,
    container: &Container<Query, Mutation, Subscription>,
) {
    let res = execute(container, request).await;
    assert_eq!(serde_json::to_string(&res).unwrap(), expected_response);
}
