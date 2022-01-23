use rusty_gql::*;

#[tokio::test]
async fn test_object_introspection() {
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
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query = r#"{ __type(name: "Cat") { kind name description fields {name description type isDeprecated} interfaces {name} } }"#;
    let req = build_test_request(query, None, Default::default());
    let expected_response = r#"{"data":{"__type":{"description":null,"fields":[{"description":null,"isDeprecated":false,"name":"name","type":{}},{"description":null,"isDeprecated":false,"name":"meows","type":{}}],"interfaces":[{"name":"Pet"}],"kind":"OBJECT","name":"Cat"}}}"#;
    check_gql_response(req, expected_response, &container).await;
}

#[tokio::test]
async fn test_interface_introspection() {
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
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query = r#"{ __type(name: "Pet") { kind name description fields {name description type isDeprecated} possibleTypes {name} } }"#;
    let req = build_test_request(query, None, Default::default());
    let expected_response = r#"{"data":{"__type":{"description":null,"fields":[{"description":null,"isDeprecated":false,"name":"name","type":{}}],"kind":"INTERFACE","name":"Pet","possibleTypes":[{"name":"Cat"},{"name":"Dog"}]}}}"#;
    check_gql_response(req, expected_response, &container).await;
}

#[tokio::test]
async fn test_input_object_introspection() {
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
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query = r#"{ __type(name: "TestInput") { kind name description inputFields {name} } }"#;
    let req = build_test_request(query, None, Default::default());
    let expected_response = r#"{"data":{"__type":{"description":null,"inputFields":[{"name":"int_field"},{"name":"str_field"}],"kind":"INPUT_OBJECT","name":"TestInput"}}}"#;
    check_gql_response(req, expected_response, &container).await;
}

#[tokio::test]
async fn test_enum_introspection() {
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
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query = r#"{ __type(name: "Country") { kind name description enumValues {name} } }"#;
    let req = build_test_request(query, None, Default::default());
    let expected_response = r#"{"data":{"__type":{"description":null,"enumValues":[{"name":"JAPAN"},{"name":"AMERICA"},{"name":"CHINA"}],"kind":"ENUM","name":"Country"}}}"#;
    check_gql_response(req, expected_response, &container).await;
}

#[tokio::test]
async fn test_union_introspection() {
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
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query = r#"{ __type(name: "Animal") { kind name description possibleTypes {name} } }"#;
    let req = build_test_request(query, None, Default::default());
    let expected_response = r#"{"data":{"__type":{"description":null,"kind":"UNION","name":"Animal","possibleTypes":[{"name":"Dog"},{"name":"Cat"}]}}}"#;
    check_gql_response(req, expected_response, &container).await;
}

#[tokio::test]
async fn test_scalar_introspection() {
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
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query = r#"{ __type(name: "DateTime") { kind name description } }"#;
    let req = build_test_request(query, None, Default::default());
    let expected_response =
        r#"{"data":{"__type":{"description":null,"kind":"SCALAR","name":"DateTime"}}}"#;
    check_gql_response(req, expected_response, &container).await;
}
