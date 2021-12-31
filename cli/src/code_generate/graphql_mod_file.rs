use crate::code_generate::FileStrategy;

use super::build_file_path_str;

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
        build_file_path_str(self.path, vec!["mod"])
    }
}
