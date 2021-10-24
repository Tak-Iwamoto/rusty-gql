use rusty_gql_codegen::GqlField;

#[GqlField(parent_type = "Query", field = "show")]
pub fn test_func(value: &str) -> String {
    value
}

#[test]
fn it_works() {
    let value = test_func("value");
    println!("{:?}", value);
}
