use rusty_gql::gql_object;

pub struct Query;

#[gql_object]
impl Query {
    async fn test(&self) -> i32 {
        12
    }

    async fn result_test(&self) -> Result<i32, String> {
        Ok(11)
    }
}

#[tokio::test]
async fn it_works() {
    let query = Query {};
    let value = query.result_test().await;
    println!("{}", &value.unwrap());
}
