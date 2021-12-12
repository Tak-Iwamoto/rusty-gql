use std::io::Cursor;

use rocket::{http::ContentType, response::Responder};

pub struct GqlResponse(pub rusty_gql::Response);

impl From<rusty_gql::Response> for GqlResponse {
    fn from(response: rusty_gql::Response) -> Self {
        GqlResponse(response)
    }
}

impl<'r> Responder<'r, 'static> for GqlResponse {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let body = serde_json::to_string(&self.0).unwrap();
        let mut res = rocket::Response::new();
        res.set_header(ContentType::new("application", "json"));
        res.set_sized_body(body.len(), Cursor::new(body));

        Ok(res)
    }
}
