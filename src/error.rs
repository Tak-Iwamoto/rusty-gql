use graphql_parser::Pos;

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct GraphQLTypedError {
    pub error_type: GqlErrorType,
    pub error_detail: Option<String>,
    pub origin: String,
    pub debug_info: String,
    pub debug_uri: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GqlError {
    pub message: String,
    pub locations: Vec<Location>,
    pub path: Vec<String>,
    pub extensions: Option<GraphQLTypedError>,
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
