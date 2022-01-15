use rusty_gql::*;
#[allow(dead_code)]

struct Test<'a> {
    test: &'a str,
}

#[Resolver]
impl<'a> Test<'a> {
    async fn value(&self) -> &'a str {
        self.test
    }
}
