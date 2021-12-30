use std::collections::BTreeMap;

use rusty_gql::*;

#[tokio::test]
pub async fn test_build_schema() {
    struct Query;

    #[Resolver]
    impl Query {
        async fn value(&self) -> i32 {
            10
        }
        async fn obj(&self) -> BTreeMap<String, i32> {
            let mut map = BTreeMap::new();
            map.insert("key1".to_string(), 1);
            map.insert("key2".to_string(), 2);
            map
        }
    }
    let contents = std::fs::read_to_string("./tests/schemas/simple_dummy.graphql").unwrap();

    let container = ArcContainer::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
    )
    .unwrap();

    let query_doc = r#"{"query": "{ value }"}"#;

    let req = serde_json::from_str::<Request>(query_doc).unwrap();
    let res = execute(&container, req).await;
    println!("{:?}", res.data);

    let obj_req = serde_json::from_str::<Request>(r#"{"query": "{ obj }"}"#).unwrap();
    let obj_res = execute(&container, obj_req).await;
    println!("{:?}", obj_res.data);
}
