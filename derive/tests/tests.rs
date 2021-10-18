use rusty_gql_derive::GQLField;

#[derive(GQLField)]
#[field_name(lowercase)]
struct Test {
    name: String,
    age: i32,
}
#[test]
fn it_works() {
    let test = Test {
        name: String::from("tom"),
        age: 27,
    };
    println!("{}", test.test());
}
