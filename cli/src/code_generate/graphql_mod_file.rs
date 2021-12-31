use crate::code_generate::FileStrategy;

use super::graphql_file_path;

pub struct ModFile {
    pub file_names: Vec<String>,
    pub base_path: String,
}

impl<'a> FileStrategy for ModFile {
    fn content(&self) -> String {
        let mut result = String::from("");
        for name in &self.file_names {
            result += format!("mod {};\n", name).as_str();
        }

        result
    }

    fn path(&self) -> String {
        graphql_file_path(vec![self.base_path.as_str(), "mod"])
    }
}
