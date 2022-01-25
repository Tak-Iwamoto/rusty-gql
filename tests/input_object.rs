use rusty_gql::*;

#[tokio::test]
pub async fn test_input_obj() {
    struct Query;

    #[derive(GqlInputObject)]
    pub struct InputObj {
        str_value: String,
        int_value: i64,
    }

    #[GqlType]
    impl Query {
        async fn input_test(&self, input: InputObj) -> String {
            format!("{}*{}", &input.str_value, &input.int_value)
        }
    }
    let contents = schema_content("./tests/schemas/input_object.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ input_test(input: {str_value: "test", int_value: 2} ) }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"input_test":"test*2"}}"#;
    check_gql_response(req, expected_response, &container).await;
}
