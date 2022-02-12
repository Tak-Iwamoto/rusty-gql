use crate::code_generate::FileDefinition;

pub struct GitignoreFile<'a> {
    pub app_name: &'a str,
}

impl<'a> FileDefinition for GitignoreFile<'a> {
    fn name(&self) -> String {
        ".gitignore".to_string()
    }

    fn path(&self) -> String {
        format!("{}/.gitignore", self.app_name)
    }

    fn content(&self) -> String {
        "/target".to_string()
    }
}
