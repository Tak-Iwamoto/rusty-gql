use std::collections::BTreeMap;

use rusty_gql::*;

#[tokio::test]
async fn test_introspection_works() {
    struct Query;

    #[GqlResolver]
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
    let contents = std::fs::read_to_string("./tests/schemas/github.graphql").unwrap();
    let query_root = QueryRoot { query: Query };

    let container = ArcContainer::new(
        contents.as_str(),
        query_root,
        EmptyMutation,
        EmptySubscription,
    )
    .unwrap();

    let user_type_query = r#"{
        __type(name: "User") {
            name
            fields {
                name
                type {
                name
                kind
                }
                __typename
            }
        }
    }"#;
    let req = Request {
        query: user_type_query.to_string(),
        operation_name: None,
        variables: Variables(BTreeMap::new()),
    };
    let res = execute(&container, req).await;
    let res_string = serde_json::to_string(&res.data).unwrap();
    println!("{:?}", res_string);

    let schema_name_query = r#"{
        __schema {
            types {
                name
            }
        }
    }"#;
    let req = Request {
        query: schema_name_query.to_string(),
        operation_name: None,
        variables: Variables(BTreeMap::new()),
    };
    let res = execute(&container, req).await;
    let res_string = serde_json::to_string(&res.data).unwrap();
    println!("{:?}", res_string);
}
