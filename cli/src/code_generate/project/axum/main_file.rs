use crate::code_generate::FileDefinition;

pub struct AxumMainFile<'a> {
    pub app_name: &'a str,
}

impl<'a> FileDefinition for AxumMainFile<'a> {
    fn name(&self) -> String {
        "main.rs".to_string()
    }

    fn path(&self) -> String {
        format!("{}/src/main.rs", self.app_name)
    }

    fn content(&self) -> String {
        main_file_content().to_string()
    }
}

fn main_file_content() -> &'static str {
    r#"mod graphql;

fn main() {
    println!("Hello, world!");
}"#
}
