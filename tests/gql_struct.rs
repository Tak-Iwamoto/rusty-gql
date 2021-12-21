use rusty_gql::GqlStruct;

struct Post {
    title: String,
    description: String,
}

#[derive(GqlStruct)]
pub struct Show {
    pub name: String,
    pub description: String,
}

#[tokio::test]
async fn it_works() {
    let show = Show {
        name: String::from("test"),
        description: String::from("test description"),
    };
    let name = show.name().await.unwrap();
    println!("{}", name);
    let des = show.description().await.unwrap();
    println!("{}", des);
}
