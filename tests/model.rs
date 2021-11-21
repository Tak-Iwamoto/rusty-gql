use rusty_gql::GqlModel;

struct Post {
    title: String,
    description: String,
}

#[derive(GqlModel)]
pub struct Show {
    pub name: String,
    pub description: String,
}

impl Show {
    fn posts() -> Vec<Post> {
        vec![Post {
            title: "post 1".to_string(),
            description: "description".to_string(),
        }]
    }
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
