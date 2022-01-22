use std::collections::BTreeMap;

use rusty_gql::*;

#[tokio::test]
pub async fn test_input_obj() {
    struct Query;

    pub struct InputObj {
        str_value: String,
        int_value: i64,
    }

    impl VariableType for InputObj {
        fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
            if let Some(GqlValue::Object(obj)) = value {
                let str_value: String = match obj.get("str_value") {
                    Some(v) => VariableType::from_gql_value(Some(v.clone())).unwrap(),
                    None => "".to_string(),
                };
                let int_value: i64 = match obj.get("int_value") {
                    Some(v) => VariableType::from_gql_value(Some(v.clone())).unwrap(),
                    None => Default::default(),
                };
                Ok(InputObj {
                    str_value,
                    int_value,
                })
            } else {
                Err("Invalid type, Expected type: object".to_string())
            }
        }

        fn into_gql_value(&self) -> GqlValue {
            let mut obj = BTreeMap::new();
            obj.insert(
                "str_value".to_string(),
                GqlValue::String(self.str_value.to_string()),
            );
            obj.insert(
                "int_value".to_string(),
                GqlValue::Number(self.int_value.into()),
            );
            GqlValue::Object(obj)
        }
    }

    #[Resolver]
    impl Query {
        async fn input_test(&self, input: InputObj) -> String {
            format!("{}*{}", &input.str_value, &input.int_value)
        }
    }
    let contents = schema_content("./tests/schemas/input_object.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ input_test(input: {str_value: "test", int_value: 2} ) }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"input_test":"test*2"}}"#;
    check_gql_response(req, expected_response, &container).await;
}
