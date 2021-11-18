use rusty_gql::GqlObject;

#[derive(Debug, GqlObject)]
pub struct Show {
    pub name: String,
    pub description: String,
}

#[tokio::test]
async fn it_works() {
    let show = Show {
        name: String::from("test"),
        description: String::from("test"),
    };
}
