use rusty_gql_codegen::gql_object;

pub struct Query;

#[gql_object]
impl Query {
    async fn test(&self) -> i32 {
        12
    }
}

#[tokio::test]
async fn it_works() {
    let query = Query{};
    let value = query.test().await;
}
