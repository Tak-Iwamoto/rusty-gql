use rusty_gql::*;

#[tokio::test]
pub async fn test_variables() {
    struct Query;

    #[GqlType]
    impl Query {
        async fn twice_value(&self, value: i32) -> i32 {
            value * 2
        }
    }
    let contents = schema_content("./tests/schemas/test_schema.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ twice_value(value: 10) }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"twice_value":20}}"#;
    check_gql_response(req, expected_response, &container).await;

    let query_doc = r#"query TestQueryWithVars($value: Int!){twice_value(value: $value)}"#;
    let variables_str = r#"{"value": 20}"#;
    let variables = serde_json::from_str::<Variables>(variables_str).unwrap();
    let req = build_test_request(query_doc, None, variables);
    let expected_response = r#"{"data":{"twice_value":40}}"#;
    check_gql_response(req, expected_response, &container).await;
}
