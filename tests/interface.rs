use rusty_gql::*;

#[tokio::test]
pub async fn test_interface() {
    struct Query;

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

    #[derive(GqlInterface)]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    #[GqlType(interface)]
    impl Animal {
        async fn name(&self) -> String {
            match self {
                Animal::Dog(obj) => obj.name.clone(),
                Animal::Cat(obj) => obj.name.clone(),
            }
        }
    }

    #[GqlType]
    impl Query {
        async fn search_animal(&self, query: String) -> Option<Animal> {
            if query.as_str() == "dog" {
                return Some(Animal::Dog(Dog {
                    name: "Pochi".to_string(),
                    woofs: true,
                }));
            } else if query.as_str() == "cat" {
                return Some(Animal::Cat(Cat {
                    name: "Tama".to_string(),
                    meow: true,
                }));
            }
            None
        }
    }
    let contents = schema_content("./tests/schemas/interface.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ search_animal(query: "dog") {
        name
        ... on Dog {
            woofs
        }
    }}"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"search_animal":{"name":"Pochi","woofs":true}}}"#;
    check_gql_response(req, expected_response, &container).await;
}
