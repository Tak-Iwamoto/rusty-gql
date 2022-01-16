use std::collections::HashMap;

use rusty_gql::*;

#[tokio::test]
pub async fn test_directive() {
    #[derive(Clone, Debug)]
    struct AuthDirective {
        requires: String,
    }

    impl AuthDirective {
        fn new() -> Box<dyn CustomDirective> {
            Box::new(AuthDirective {
                requires: "ADMIN".to_string(),
            })
        }
    }

    #[async_trait::async_trait]
    impl CustomDirective for AuthDirective {
        async fn resolve_field(
            &self,
            ctx: &FieldContext<'_>,
            resolve_fut: ResolveFut<'_>,
        ) -> ResolverResult<Option<GqlValue>> {
            println!("call custom directive");
            resolve_fut.await.map(|v| {
                if let Some(v) = v {
                    if self.requires == "ADMIN".to_string() {
                        None
                    } else {
                        Some(v)
                    }
                } else {
                    v
                }
            })
        }
    }
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
            self.age
        }
    }

    struct Query;

    #[Resolver]
    impl Query {
        async fn persons(&self) -> Vec<Person> {
            vec![
                Person {
                    name: "Tom".to_string(),
                    description: Some("test person".to_string()),
                    age: 20,
                },
                Person {
                    name: "Mary".to_string(),
                    description: Some("test person mary".to_string()),
                    age: 28,
                },
            ]
        }

        async fn person(&self, id: ID) -> Person {
            Person {
                name: "Tom".to_string(),
                description: Some("test person".to_string()),
                age: 20,
            }
        }
    }
    let contents = schema_content("./tests/schemas/custom_directive.graphql");

    let mut custom_directive_maps = HashMap::new();
    custom_directive_maps.insert("auth", AuthDirective::new());

    let container = Container::new(
        &vec![contents.as_str()],
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        custom_directive_maps,
    )
    .unwrap();

    let query_doc = r#"{ person(id: 1) {name age} }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"person":{"age":null,"name":null}}}"#;
    check_gql_response(req, expected_response, &container).await;
}
