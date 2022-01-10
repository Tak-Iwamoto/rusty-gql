use std::collections::BTreeMap;

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
    let contents = schema_content("./tests/schemas/github.graphql");

    let container = ArcContainer::new(
        &vec![contents.as_str()],
        QueryRoot { query: Query },
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
    println!("{:?}", res);
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
}
