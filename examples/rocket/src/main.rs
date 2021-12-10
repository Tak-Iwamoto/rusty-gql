use rusty_gql::{execute, ArcContainer, Request, Resolver, Response};

fn main() {
    println!("Hello, world!");
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn execute_gql<T: Resolver>(container: &ArcContainer<T>, request: Request) -> Response {
    execute(container, request).await
}
