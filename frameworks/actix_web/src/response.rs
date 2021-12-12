use actix_web::{
    http::{Error, StatusCode},
    HttpResponse, Responder,
};
use futures::future::{ready, Ready};

pub struct GqlResponse(pub rusty_gql::Response);

impl From<rusty_gql::Response> for GqlResponse {
    fn from(response: rusty_gql::Response) -> Self {
        GqlResponse(response)
    }
}

impl Responder for GqlResponse {
    type Body = HttpResponse;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self.0).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
