#![allow(warnings, unused)]
use std::collections::{BTreeMap, HashMap};

use rusty_gql::*;

#[tokio::test]
pub async fn test_directive() {
    #[derive(Clone, Debug)]
    struct auth;

    impl auth {
        fn new() -> Box<dyn CustomDirective> {
            Box::new(auth {})
        }
    }

    #[async_trait::async_trait]
    impl CustomDirective for auth {
        async fn resolve_field(
            &self,
            _ctx: &Context<'_>,
            directive_args: &BTreeMap<String, GqlValue>,
            resolve_fut: ResolveFut<'_>,
        ) -> ResolverResult<Option<GqlValue>> {
            resolve_fut.await.map(|v| {
                if let Some(GqlValue::Enum(arg_value)) = directive_args.get("requires") {
                    if arg_value == "ADMIN" {
                        return None;
                    } else {
                        v
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

    struct Query;

    #[GqlType]
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

        #[allow(unused)]
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
    custom_directive_maps.insert("auth", auth::new());

    let container = Container::new(
        &vec![contents.as_str()],
        Query,
        EmptyMutation,
        EmptySubscription,
        custom_directive_maps,
    )
    .unwrap();

    let query_doc = r#"{ person(id: 1) {name age} }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"person":{"age":20,"name":null}}}"#;
    check_gql_response(req, expected_response, &container).await;
}
