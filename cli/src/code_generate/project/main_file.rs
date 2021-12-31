use crate::code_generate::FileStrategy;

pub struct MainFile<'a> {
    pub app_name: &'a str,
}

impl<'a> FileStrategy for MainFile<'a> {
    fn path(&self) -> String {
        format!("{}/src/main.rs", self.app_name)
    }

    fn content(&self) -> String {
        main_file_content().to_string()
    }
}

fn main_file_content() -> &'static str {
    r#"fn main() {
    println!("Hello, world!");
}"#
}
