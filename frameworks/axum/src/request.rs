use axum::body::HttpBody;
use axum::extract::FromRequest;
use axum::http::Method;

pub struct GqlRequest(pub rusty_gql::Request);

impl<B> FromRequest<B> for GqlRequest
where
    B: HttpBody,
{
    type Rejection;

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        if &Method::GET == req.method() {

        } else if &Method::POST == req.method() {

        }
    }
}
