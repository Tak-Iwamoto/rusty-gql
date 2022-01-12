use std::collections::BTreeMap;

use rusty_gql::*;

#[tokio::test]
pub async fn test_object() {
    struct Person {
        name: String,
        description: Option<String>,
        age: i32,
    }

    #[Resolver]
    impl Person {
        async fn name(&self) -> String {
            self.name.clone()
        }
        async fn description(&self) -> Option<String> {
            self.description.clone()
        }
        async fn age(&self) -> i32 {
            self.age.clone()
        }
    }

    struct Query;

    #[Resolver]
    impl Query {
        async fn obj(&self) -> BTreeMap<String, i32> {
            let mut map = BTreeMap::new();
            map.insert("key1".to_string(), 1);
            map.insert("key2".to_string(), 2);
            map
        }

        async fn person(&self, id: ID) -> Person {
            let person = Person {
                name: "Tom".to_string(),
                description: Some("description".to_string()),
                age: 20,
            };
            person
        }
    }

    let contents = schema_content("./tests/schemas/test_schema.graphql");

    let container = ArcContainer::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
    )
    .unwrap();

    let partly_person_query = r#"{ person(id: 1) { ... on Person { name age } } }"#;
    let req = build_test_request(partly_person_query, None, Default::default());
    let expected = r#"{"data":{"person":{"age":20,"name":"Tom"}}}"#;
    check_gql_response(req, expected, &container).await;
}
