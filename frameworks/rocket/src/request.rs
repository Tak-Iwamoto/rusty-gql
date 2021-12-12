use rocket::{
    data::{self, FromData, ToByteUnit},
    http::Status,
};
use rusty_gql::{receive_http_request, HttpRequestError};
use tokio_util::compat::TokioAsyncReadCompatExt;

pub struct GqlRequest(pub rusty_gql::Request);

#[rocket::async_trait]
impl<'r> FromData<'r> for GqlRequest {
    type Error = HttpRequestError;

    async fn from_data(
        req: &'r rocket::Request<'_>,
        data: rocket::Data<'r>,
    ) -> rocket::data::Outcome<'r, Self> {
        let req = receive_http_request(
            data.open(
                req.limits()
                    .get("graphql")
                    .unwrap_or_else(|| 128.kibibytes()),
            )
            .compat(),
        )
        .await;

        match req {
            Ok(request) => data::Outcome::Success(Self(request)),
            Err(e) => data::Outcome::Failure((Status::BadRequest, e)),
        }
    }
}
