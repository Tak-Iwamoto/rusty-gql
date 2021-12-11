use axum::response::IntoResponse;

pub struct GqlResponse(pub rusty_gql::Response);

impl From<rusty_gql::Response> for GqlResponse {
    fn from(response: rusty_gql::Response) -> Self {
        GqlResponse(response)
    }
}

impl IntoResponse for GqlResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(&self.0).into_response()
    }
}
