use std::collections::HashMap;

use futures_util::{pin_mut, AsyncRead, AsyncReadExt};
use serde::{Deserialize, Serialize};

use crate::{variables::Variables, GqlValue};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub operation_name: Option<String>,
    #[serde(default)]
    pub variables: Variables,
    #[serde(default)]
    pub extensions: HashMap<String, GqlValue>,
}

pub async fn receive_http_request(
    body: impl AsyncRead + Send,
) -> Result<Request, HttpRequestError> {
    receive_json_body(body).await
}

pub async fn receive_json_body(body: impl AsyncRead) -> Result<Request, HttpRequestError> {
    let mut data = Vec::new();
    pin_mut!(body);

    body.read_to_end(&mut data)
        .await
        .map_err(HttpRequestError::Io)?;
    Ok(serde_json::from_slice::<Request>(&data)
        .map_err(|err| HttpRequestError::InvalidRequest(Box::new(err)))?)
}

#[derive(Debug)]
pub enum HttpRequestError {
    Io(std::io::Error),
    InvalidRequest(Box<dyn std::error::Error + Send + Sync>),
}

#[cfg(test)]
mod tests {
    use serde_json::Number;

    use crate::{GqlValue, Request};

    #[test]
    fn test_operation_name() {
        let query_doc = r#"{"query": "{ hero droids jedi }", "operationName": "hero"}"#;
        let req = serde_json::from_str::<Request>(query_doc).unwrap();
        assert_eq!(req.query, "{ hero droids jedi }");
        assert_eq!(req.operation_name, Some("hero".to_string()));
        assert!(req.variables.0.is_empty());
    }

    #[test]
    fn test_variables() {
        let query_doc = r#"{"query": "{ hero droids jedi }", "variables": {"var1": 100, "var2": "value", "var3": [1,1,1]}}"#;
        let req = serde_json::from_str::<Request>(query_doc).unwrap();
        assert_eq!(req.query, "{ hero droids jedi }");
        assert!(req.operation_name.is_none());

        assert_eq!(
            req.variables.0.get("var1"),
            Some(&GqlValue::Number(Number::from(100 as i32)))
        );
        assert_eq!(
            req.variables.0.get("var2"),
            Some(&GqlValue::String(String::from("value")))
        );
        assert_eq!(
            req.variables.0.get("var3"),
            Some(&GqlValue::List(vec![
                GqlValue::Number(Number::from(1 as i32)),
                GqlValue::Number(Number::from(1 as i32)),
                GqlValue::Number(Number::from(1 as i32))
            ]))
        );
    }

    #[test]
    fn test_null_variables() {
        let query_doc = r#"{"query": "{ hero droids jedi }", "variables": null}"#;
        let req = serde_json::from_str::<Request>(query_doc).unwrap();
        assert_eq!(req.query, "{ hero droids jedi }");
        assert!(req.operation_name.is_none());
        assert!(req.variables.0.is_empty());
    }
}
