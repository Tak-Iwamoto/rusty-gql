use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::{error::GqlError, GqlValue};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Response {
    #[serde(default)]
    pub data: GqlValue,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<GqlError>,
    #[serde(skip)]
    pub http_headers: HeaderMap<String>,
}

impl Response {
    pub fn new(data: impl Into<GqlValue>) -> Self {
        Self {
            data: data.into(),
            errors: vec![],
            http_headers: Default::default(),
        }
    }

    pub fn from_errors(errors: Vec<GqlError>) -> Self {
        Self {
            errors,
            data: Default::default(),
            http_headers: Default::default(),
        }
    }

    pub fn from_data_and_errors(data: impl Into<GqlValue>, errors: Vec<GqlError>) -> Self {
        Self {
            data: data.into(),
            errors,
            http_headers: Default::default(),
        }
    }

    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn is_error(&self) -> bool {
        !self.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::Number;

    use crate::{GqlValue, Response};

    #[test]
    fn test_json_serialize() {
        let boolean = Response::new(GqlValue::Boolean(true));
        assert_eq!(serde_json::to_string(&boolean).unwrap(), r#"{"data":true}"#);

        let map = BTreeMap::from([
            ("a".to_string(), GqlValue::Number(Number::from(1))),
            ("b".to_string(), GqlValue::Number(Number::from(2))),
        ]);
        let obj = Response::new(GqlValue::Object(map));
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"data":{"a":1,"b":2}}"#
        );
    }
}
