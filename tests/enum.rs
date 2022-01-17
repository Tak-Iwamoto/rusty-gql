use rusty_gql::*;

#[tokio::test]
pub async fn test_enum() {
    struct Query;

    enum SampleEnum {
        Value0,
        Value1,
    }

    #[async_trait::async_trait]
    impl FieldResolver for SampleEnum {
        async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
            match self {
                SampleEnum::Value0 => Ok(Some(GqlValue::String("Value0".to_string()))),
                SampleEnum::Value1 => Ok(Some(GqlValue::String("Value1".to_string()))),
            }
        }

        fn type_name() -> String {
            "SampleEnum".to_string()
        }
    }

    #[async_trait::async_trait]
    impl SelectionSetResolver for SampleEnum {
        async fn resolve_selection_set(
            &self,
            ctx: &SelectionSetContext<'_>,
        ) -> ResolverResult<GqlValue> {
            match self {
                SampleEnum::Value0 => Ok(GqlValue::String("Value0".to_string())),
                SampleEnum::Value1 => Ok(GqlValue::String("Value1".to_string())),
            }
        }
    }

    #[Resolver]
    impl Query {
        async fn enum_value(&self) -> SampleEnum {
            SampleEnum::Value0
        }
    }
    let contents = schema_content("./tests/schemas/enum.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ enum_value }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"enum_value":"Value0"}}"#;
    check_gql_response(req, expected_response, &container).await;
}
