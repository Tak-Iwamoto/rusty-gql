#[derive(Clone)]
pub struct GraphQLPath {
    pub prev: Option<Box<GraphQLPath>>,
    pub key: String,
    pub typename: String,
}

impl Default for GraphQLPath {
    fn default() -> Self {
        Self {
            prev: None,
            key: String::from(""),
            typename: String::from("Query"),
        }
    }
}

impl GraphQLPath {
    pub fn prev(mut self, prev: Option<GraphQLPath>) -> Self {
        self.prev = prev.map_or(None, |p| Some(Box::new(p)));
        self
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = key;
        self
    }

    pub fn typename(mut self, typename: String) -> Self {
        self.typename = typename;
        self
    }
}
