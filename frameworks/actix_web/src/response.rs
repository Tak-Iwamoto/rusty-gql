use actix_web::{
    http::{Error, StatusCode},
    HttpResponse, Responder,
};
use futures::future::{ready, Ready};

pub struct ActixWebGqlResponse(pub rusty_gql::Response);

impl From<rusty_gql::Response> for ActixWebGqlResponse {
    fn from(response: rusty_gql::Response) -> Self {
        ActixWebGqlResponse(response)
    }
}

impl Responder for ActixWebGqlResponse {
    type Error = Error;

    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _: &actix_web::HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self.0).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
