use rusty_gql::*;

#[tokio::test]
pub async fn test_build_schema() {
    struct Query;

    #[GqlResolver]
    impl Query {
        async fn value(&self, _ctx: &FieldContext<'_>) -> i32 {
            10
        }
    }
    let contents = std::fs::read_to_string("./tests/schemas/simple_dummy.graphql").unwrap();

    let container =
        ArcContainer::new(contents.as_str(), Query, EmptyMutation, EmptySubscription).unwrap();

    let query_doc = r#"{"query": "{ value }"}"#;

    let req = serde_json::from_str::<Request>(query_doc).unwrap();
    let res = execute(&container, req).await;
}
