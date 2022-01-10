use std::collections::BTreeMap;

use rusty_gql::*;

#[tokio::test]
pub async fn test_int() {
    struct Query;

    #[Resolver]
    impl Query {
        async fn value(&self) -> i32 {
            10
        }
    }
    let contents = schema_content("./tests/schemas/simple_dummy.graphql");

    let container = ArcContainer::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
    )
    .unwrap();

    let query_doc = r#"{"query": "{ value }"}"#;
    let expected_response = r#"{"data":{"value":10}}"#;
    check_gql_response(query_doc, expected_response, &container).await;
}

#[tokio::test]
pub async fn test_object() {
    struct Query;

    #[Resolver]
    impl Query {
        async fn obj(&self) -> BTreeMap<String, i32> {
            let mut map = BTreeMap::new();
            map.insert("key1".to_string(), 1);
            map.insert("key2".to_string(), 2);
            map
        }
    }

    let contents = schema_content("./tests/schemas/simple_dummy.graphql");

    let container = ArcContainer::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
    )
    .unwrap();
    let query_doc = r#"{"query": "{ obj { key1 key2} }"}"#;
    let expected_response = r#"{"data":{"obj":{"key1":1,"key2":2}}}"#;
    check_gql_response(query_doc, expected_response, &container).await;
}
