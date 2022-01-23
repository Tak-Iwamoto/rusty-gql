use rusty_gql::*;

#[tokio::test]
pub async fn test_union() {
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

    struct Dog {
        name: String,
        woofs: bool,
    }

    #[GqlType]
    impl Dog {
        async fn name(&self) -> String {
            self.name.clone()
        }
        async fn woofs(&self) -> bool {
            self.woofs
        }
    }

    struct Cat {
        name: String,
        meow: bool,
    }

    #[GqlType]
    impl Cat {
        async fn name(&self) -> String {
            self.name.clone()
        }
        async fn meow(&self) -> bool {
            self.meow
        }
    }

    #[derive(GqlUnion)]
    enum SearchAnimal {
        Person(Person),
        Dog(Dog),
        Cat(Cat),
    }

    #[GqlType]
    impl Query {
        async fn search_animal(&self, query: String) -> Option<SearchAnimal> {
            if query.as_str() == "dog" {
                return Some(SearchAnimal::Dog(Dog {
                    name: "Pochi".to_string(),
                    woofs: true,
                }));
            } else if query.as_str() == "cat" {
                return Some(SearchAnimal::Cat(Cat {
                    name: "Tama".to_string(),
                    meow: true,
                }));
            } else if query.as_str() == "person" {
                return Some(SearchAnimal::Person(Person {
                    name: "Tom".to_string(),
                    description: None,
                    age: 20,
                }));
            }
            None
        }
    }
    let contents = schema_content("./tests/schemas/union.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ search_animal(query: "dog") {
        ... on Dog {
            name
            woofs
        }
        ... on Cat {
            name
            meows
        }
        ... on Person {
            name
            age
        }
    }}"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"search_animal":{"name":"Pochi","woofs":true}}}"#;
    check_gql_response(req, expected_response, &container).await;

    let query_doc = r#"{ search_animal(query: "person") {
        ... on Person {
            name
            age
        }
    }}"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"search_animal":{"age":20,"name":"Tom"}}}"#;
    check_gql_response(req, expected_response, &container).await;
}
