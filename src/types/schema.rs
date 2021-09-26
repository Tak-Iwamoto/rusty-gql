pub struct Schema {
    extensions: Vec<String>,
    query: String,
    mutation: String,
    subscription: String,
}

impl Schema {
    pub fn execute(&self) {}
}
