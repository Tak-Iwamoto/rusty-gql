use std::pin::Pin;

use actix_web::{
    http::{Error, Method},
    FromRequest, Result,
};
use futures::Future;

pub struct GqlRequest(pub rusty_gql::Request);

impl FromRequest for GqlRequest {
    type Error;

    type Future;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        if req.method() == Method::GET {
            let body = serde_urlencoded::from_str(req.query_string());
            Box::pin(async move { Ok(Self(rusty_gql::Request(body?))) })
        }
    }
}
