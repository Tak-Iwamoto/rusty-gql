use crate::{execute, ArcContainer, FieldResolver, Request};

pub fn schema_content(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub async fn check_gql_response<
    Query: FieldResolver,
    Mutation: FieldResolver,
    Subscription: FieldResolver,
>(
    query_doc: &str,
    expected_response: &str,
    container: &ArcContainer<Query, Mutation, Subscription>,
) {
    let req = serde_json::from_str::<Request>(query_doc).unwrap();
    let res = execute(&container, req).await;
    assert_eq!(serde_json::to_string(&res).unwrap(), expected_response);
}
