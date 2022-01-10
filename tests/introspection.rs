use rusty_gql::*;

#[tokio::test]
async fn test_introspection_works() {
    struct Query;

    #[Resolver]
    impl Query {
        async fn value(&self) -> i32 {
            10
        }
    }
    let contents = schema_content("./tests/schemas/pet_schema.graphql");

    let container = ArcContainer::new(
        &vec![contents.as_str()],
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
    )
    .unwrap();

    let query = r#"{ __type(name: "Cat") { name } }"#;
    let req = build_test_request(query, None, Default::default());
    let expected_response = r#"{"data":{"__type":{"name":"Cat"}}}"#;
    check_gql_response(req, expected_response, &container).await;

    let query = r#"{ __schema { types { name } } }"#;
    let req = build_test_request(query, None, Default::default());
    let expected_response = r#"{"data":{"__schema":{"types":[{"name":"Boolean"},{"name":"Cat"},{"name":"Dog"},{"name":"Float"},{"name":"ID"},{"name":"Int"},{"name":"Pet"},{"name":"Query"},{"name":"String"}]}}}"#;
    check_gql_response(req, expected_response, &container).await;
}
