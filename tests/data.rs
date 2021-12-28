use rusty_gql::*;

#[tokio::test]
async fn it_works() {
    pub struct Person {
        pub name: String,
        pub description: String,
        pub age: i32,
    }

    #[GqlResolver]
    impl Person {
        async fn name(&self) -> String {
            self.name.clone()
        }
        async fn description(&self) -> String {
            self.description.clone()
        }
        async fn age(&self) -> i32 {
            self.age
        }
    }

    struct Query;

    #[GqlResolver]
    impl Query {
        async fn person(&self, ctx: &FieldContext<'_>) -> Person {
            let person = Person {
                name: String::from("test"),
                description: String::from("test description"),
                age: 32,
            };
            person
        }
    }

    let contents = std::fs::read_to_string("./tests/schemas/simple_dummy.graphql").unwrap();

    let container =
        ArcContainer::new(contents.as_str(), Query, EmptyMutation, EmptySubscription).unwrap();

    let name_query = r#"{"query": "{ person { name } }"}"#;
    let name_req = serde_json::from_str::<Request>(name_query).unwrap();
    let name_res = execute(&container, name_req).await;
    println!("{:?}", name_res.data);

    let description_query = r#"{"query": "{ person { description } }"}"#;
    let des_req = serde_json::from_str::<Request>(description_query).unwrap();
    let des_res = execute(&container, des_req).await;
    println!("{:?}", des_res.data);
}
