use axum::body::{self, Body, HttpBody};
use axum::extract::FromRequest;
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use rusty_gql::HttpRequestError;

pub struct GqlRequest(pub rusty_gql::Request);

pub struct GqlRejection(pub HttpRequestError);

impl IntoResponse for GqlRejection {
    fn into_response(self) -> Response {
        let body = body::boxed(body::Full::from(format!("{:?}", self.0)));
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(body)
            .unwrap()
    }
}

impl From<HttpRequestError> for GqlRejection {
    fn from(error: HttpRequestError) -> Self {
        GqlRejection(error)
    }
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for GqlRequest
where
    B: HttpBody + Send + Sync + 'static,
{
    type Rejection = GqlRejection;

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        if &Method::GET == req.method() {
            let res =
                serde_urlencoded::from_str(req.uri().query().unwrap_or_default()).map_err(|err| {
                    HttpRequestError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("failed to parse graphql requst from query params: {}", err),
                    ))
                });
            Ok(Self(res?))
        } else {
            let res =
                serde_urlencoded::from_str(req.uri().query().unwrap_or_default()).map_err(|err| {
                    HttpRequestError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("failed to parse graphql requst from query params: {}", err),
                    ))
                });
            Ok(Self(res?))
        }
    }
}
