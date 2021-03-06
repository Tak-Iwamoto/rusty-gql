use std::collections::BTreeMap;

use rusty_gql::*;

#[tokio::test]
pub async fn test_int() {
    struct Query;

    #[GqlType]
    impl Query {
        async fn value(&self) -> i32 {
            10
        }
    }
    let contents = schema_content("./tests/schemas/test_schema.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ value }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"value":10}}"#;
    check_gql_response(req, expected_response, &container).await;
}

#[tokio::test]
pub async fn test_object() {
    struct Person {
        name: String,
        description: Option<String>,
        age: i32,
    }

    #[GqlType]
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

    #[GqlType]
    impl Query {
        async fn obj(&self) -> BTreeMap<String, i32> {
            let mut map = BTreeMap::new();
            map.insert("key1".to_string(), 1);
            map.insert("key2".to_string(), 2);
            map
        }

        #[allow(unused)]
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

    let container = Container::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let obj_query = r#"{ obj { key1 key2 } }"#;
    let req = build_test_request(obj_query, None, Default::default());
    let expected = r#"{"data":{"obj":{"key1":1,"key2":2}}}"#;
    check_gql_response(req, expected, &container).await;

    let person_query = r#"{ person(id: 1) { name age description } }"#;
    let req = build_test_request(person_query, None, Default::default());
    let expected = r#"{"data":{"person":{"age":20,"description":"description","name":"Tom"}}}"#;
    check_gql_response(req, expected, &container).await;

    let partly_person_query = r#"{ person(id: 1) { name age } }"#;
    let req = build_test_request(partly_person_query, None, Default::default());
    let expected = r#"{"data":{"person":{"age":20,"name":"Tom"}}}"#;
    check_gql_response(req, expected, &container).await;
}

#[tokio::test]
pub async fn test_list() {
    struct Query;
    struct Person {
        name: String,
        description: Option<String>,
        age: i32,
    }

    #[GqlType]
    impl Person {
        async fn name(&self) -> String {
            self.name.clone()
        }
        async fn description(&self) -> Option<String> {
            self.description.clone()
        }
        async fn age(&self) -> i32 {
            self.age
        }
    }

    #[GqlType]
    impl Query {
        async fn persons(&self) -> Vec<Person> {
            vec![
                Person {
                    name: "Tom".to_string(),
                    description: None,
                    age: 20,
                },
                Person {
                    name: "Mary".to_string(),
                    description: Some("sample data".to_string()),
                    age: 10,
                },
            ]
        }
    }
    let contents = schema_content("./tests/schemas/test_schema.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ persons {name age} }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response =
        r#"{"data":{"persons":[{"age":20,"name":"Tom"},{"age":10,"name":"Mary"}]}}"#;
    check_gql_response(req, expected_response, &container).await;
}
