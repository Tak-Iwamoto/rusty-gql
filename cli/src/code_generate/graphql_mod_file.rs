use crate::code_generate::FileStrategy;

use super::concat_file_path;

pub struct GqlModFile<'a> {
    pub file_names: Vec<String>,
    pub path: &'a str,
}

impl<'a> FileStrategy for GqlModFile<'a> {
    fn content(&self) -> String {
        let mut result = String::from("");
        for name in &self.file_names {
            result += format!("mod {};\n", name).as_str();
        }

        result
    }

    fn path(&self) -> String {
        concat_file_path(self.path, vec!["mod"])
    }
}
