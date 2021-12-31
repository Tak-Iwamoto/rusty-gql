use crate::code_generate::FileDefinition;

use super::build_file_path_str;

pub struct ModFile<'a> {
    pub file_names: Vec<String>,
    pub path: &'a str,
}

impl<'a> FileDefinition for ModFile<'a> {
    fn content(&self) -> String {
        let mut result = String::from("");
        for name in &self.file_names {
            result += format!("mod {};\n", name).as_str();
        }

        result
    }

    fn path(&self) -> String {
        build_file_path_str(self.path, vec!["mod"])
    }
}
