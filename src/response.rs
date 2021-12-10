use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::{error::GqlError, GqlValue};

#[derive(Serialize, Deserialize, Default)]
pub struct Response {
    #[serde(default)]
    data: GqlValue,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<GqlError>,
    #[serde(skip)]
    pub http_headers: HeaderMap<String>,
}

impl Response {
    pub fn new(data: impl Into<GqlValue>) -> Self {
        Self {
            data: data.into(),
            ..Default::default()
        }
    }

    pub fn from_errors(errors: Vec<GqlError>) -> Self {
        Self {
            errors,
            ..Default::default()
        }
    }

    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn is_error(&self) -> bool {
        !self.is_ok()
    }
}
