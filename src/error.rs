#[derive(Debug, Clone)]
pub struct Location {
    pub line: i32,
    pub column: i32,
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
