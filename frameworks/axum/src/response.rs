use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;

pub struct AxumGqlResponse(pub rusty_gql::Response);

impl From<rusty_gql::Response> for AxumGqlResponse {
    fn from(response: rusty_gql::Response) -> Self {
        AxumGqlResponse(response)
    }
}

impl IntoResponse for AxumGqlResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(&self.0).into_response()
    }
}
