use super::model::Post;

pub struct Query;

impl Query {
    pub async fn posts(&self) -> Vec<Post> {
        vec![Post {
            title: "name1".to_string(),
            description: "des".to_string(),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::Query;

    #[tokio::test]
    async fn it_works() {
        let query = Query {};
        println!("{:?}", query.posts().await);
    }
}
