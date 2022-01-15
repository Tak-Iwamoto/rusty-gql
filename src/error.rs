use std::fmt::{self, Debug, Display, Formatter};

use graphql_parser::Pos;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum GqlErrorType {
    BadRequest,
    FailedPreCondition,
    Internal,
    NotFound,
    PermissionDenied,
    Unauthenticated,
    Unavailable,
    Unknow,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct GqlTypedError {
    pub error_type: GqlErrorType,
    pub error_detail: Option<String>,
    pub origin: String,
    pub debug_info: String,
    pub debug_uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GqlError {
    pub message: String,
    pub locations: Vec<Location>,
    pub path: Vec<String>,
    pub extensions: Option<GqlTypedError>,
}

impl GqlError {
    pub fn new(message: impl Into<String>, pos: Option<Pos>) -> Self {
        GqlError {
            message: message.into(),
            locations: pos
                .map(|pos| {
                    vec![Location {
                        line: pos.line,
                        column: pos.column,
                    }]
                })
                .unwrap_or_default(),
            path: Vec::new(),
            extensions: None,
        }
    }
}

impl Display for GqlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GqlError")
            .field("message", &self.message)
            .field("locations", &self.locations)
            .field("path", &self.path)
            .field("extensions", &self.extensions)
            .finish()
    }
}

pub struct ErrorWrapper {
    pub message: String,
    pub extensions: Option<GqlTypedError>,
}

impl ErrorWrapper {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            extensions: None,
        }
    }
    pub fn into_gql_error(self, pos: Pos) -> GqlError {
        GqlError {
            message: self.message,
            locations: vec![Location {
                line: pos.line,
                column: pos.column,
            }],
            path: Vec::new(),
            extensions: self.extensions,
        }
    }
}

impl<T: Display + Send + Sync + 'static> From<T> for ErrorWrapper {
    fn from(err: T) -> Self {
        Self {
            message: err.to_string(),
            extensions: None,
        }
    }
}

impl Debug for ErrorWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorWrapper")
            .field("message", &self.message)
            .finish()
    }
}

impl PartialEq for ErrorWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.message.eq(&other.message) && self.extensions.eq(&other.extensions)
    }
}
