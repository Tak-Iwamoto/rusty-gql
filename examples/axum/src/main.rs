use std::{convert::Infallible, net::SocketAddr};
use askama::Template;
use rusty_gql::GraphiQLTemplate;

use axum:: {Router, body::{Bytes, Full}, handler::{get}, http::{Response, StatusCode}, response::{Html, IntoResponse}};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(graphiql_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn graphiql_handler() -> impl IntoResponse {
    HtmlTemplate(GraphiQLTemplate {})
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    type Body = Full<Bytes>;
    type BodyError = Infallible;
    fn into_response(self) -> axum::http::Response<Self::Body> {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!("Failed to start GraphQL: {}", err)))
                .unwrap(),
        }
    }
}
