use crate::code_generate::FileDefinition;

pub struct TodoSchemaFile<'a> {
    pub app_name: &'a str,
}
impl<'a> FileDefinition for TodoSchemaFile<'a> {
    fn name(&self) -> String {
        "schema.graphql".to_string()
    }

    fn path(&self) -> String {
        format!("{}/schema/schema.graphql", self.app_name)
    }

    fn content(&self) -> String {
        todo_schema_content().to_string()
    }
}

fn todo_schema_content() -> &'static str {
    r#"type Query {
  todos(first: Int): [Todo!]!
}

type Todo {
  title: String!
  content: String
  done: Boolean!
}
"#
}
