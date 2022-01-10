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
        QueryRoot { query: Query },
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

    let contents = schema_content("./tests/schemas/simple_dummy.graphql");

    let container = ArcContainer::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
    )
    .unwrap();

    let obj_query = r#"{"query": "{ obj { key1 key2} }"}"#;
    let expected = r#"{"data":{"obj":{"key1":1,"key2":2}}}"#;
    check_gql_response(obj_query, expected, &container).await;

    let person_query = r#"{"query": "{ person(id: 1) { name age description } }"}"#;
    let expected = r#"{"data":{"person":{"age":20,"description":"description","name":"Tom"}}}"#;
    check_gql_response(person_query, expected, &container).await;

    let partly_person_query = r#"{"query": "{ person(id: 1) { name age } }"}"#;
    let expected = r#"{"data":{"person":{"age":20,"name":"Tom"}}}"#;
    check_gql_response(partly_person_query, expected, &container).await;
}
