#[allow(dead_code)]
use rusty_gql::*;

#[tokio::test]
async fn test_inline_framgnet() {
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

    let query = r#"{ person(id: 1) { ... on Person {name, age, description} } }"#;
    let req = build_test_request(query, None, Default::default());
    let expected = r#"{"data":{"person":{"age":20,"description":"description","name":"Tom"}}}"#;
    check_gql_response(req, expected, &container).await;
}

#[tokio::test]
async fn test_framgnet_spread() {
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

    let query = r#"
    query {
        person(id: 1) {
            ...PersonFragment
        }
    }

    fragment PersonFragment on Person {
        name age
    }
    "#;
    let req = build_test_request(query, None, Default::default());
    let expected = r#"{"data":{"person":{"age":20,"name":"Tom"}}}"#;
    check_gql_response(req, expected, &container).await;
}
