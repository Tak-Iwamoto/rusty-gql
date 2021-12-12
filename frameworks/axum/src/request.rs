use axum::extract::{BodyStream, FromRequest};
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{body, BoxError};
use bytes::Bytes;
use futures::TryStreamExt;
use rusty_gql::{receive_http_request, HttpRequestError};
use tokio_util::compat::TokioAsyncReadCompatExt;

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
    B: http_body::Body + Unpin + Send + Sync + 'static,
    B::Data: Into<Bytes>,
    B::Error: Into<BoxError>,
{
    type Rejection = GqlRejection;
    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        if let (&Method::GET, uri) = (req.method(), req.uri()) {
            let res = serde_urlencoded::from_str(uri.query().unwrap_or_default()).map_err(|err| {
                HttpRequestError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("failed to parse graphql requst from query params: {}", err),
                ))
            });
            Ok(Self(res?))
        } else {
            let body_stream = BodyStream::from_request(req)
                .await
                .map_err(|err| {
                    HttpRequestError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        err.to_string(),
                    ))
                })?
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()));
            let body_reader = tokio_util::io::StreamReader::new(body_stream).compat();
            Ok(Self(receive_http_request(body_reader).await?))
        }
    }
}
