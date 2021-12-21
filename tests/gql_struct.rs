use rusty_gql::GqlStruct;

struct Post {
    title: String,
    description: String,
}

#[derive(GqlStruct)]
pub struct Person {
    pub name: String,
    pub description: String,
    pub age: i32,
}

#[tokio::test]
async fn it_works() {
    let person = Person {
        name: String::from("test"),
        description: String::from("test description"),
        age: 32,
    };
    let name = person.name().await.unwrap();
    let des = person.description().await.unwrap();
    println!("{}", des);
}
