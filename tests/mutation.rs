#![allow(warnings, unused)]
use rusty_gql::*;

#[tokio::test]
pub async fn test_mutation() {
    struct Query;

    #[GqlType]
    impl Query {
        async fn value(&self) -> i32 {
            10
        }
    }

    struct Mutation;

    #[derive(GqlInputObject)]
    struct MutationInput {
        value: String,
    }

    #[GqlType]
    impl Mutation {
        async fn testMutation(&self, input: MutationInput) -> bool {
            true
        }
    }
    let contents = schema_content("./tests/schemas/test_schema.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        Query,
        Mutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"mutation { testMutation(input: {value: "test"}) }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"testMutation":true}}"#;
    check_gql_response(req, expected_response, &container).await;
}
