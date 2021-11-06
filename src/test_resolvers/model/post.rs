use super::Author;

#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub description: String,
}

impl Post {
    pub async fn authors(&self) -> Vec<Author> {
        vec![Author {
            name: "Tom".to_string(),
            age: 32,
        }]
    }
}
