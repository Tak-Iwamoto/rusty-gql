use rusty_gql::*;

#[tokio::test]
pub async fn test_enum() {
    struct Query;

    #[derive(GqlEnum)]
    enum SampleEnum {
        Value0,
        #[allow(unused)]
        Value1,
    }

    #[GqlType]
    impl Query {
        async fn enum_value(&self) -> SampleEnum {
            SampleEnum::Value0
        }
    }
    let contents = schema_content("./tests/schemas/enum.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ enum_value }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"enum_value":"Value0"}}"#;
    check_gql_response(req, expected_response, &container).await;
}
