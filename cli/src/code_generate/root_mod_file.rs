use heck::ToSnakeCase;

use crate::code_generate::FileDefinition;

use super::path_str;

pub struct RootModFile<'a> {
    pub file_names: Vec<String>,
    pub path: &'a str,
}

impl<'a> FileDefinition for RootModFile<'a> {
    fn content(&self) -> String {
        let mut mod_str = String::from("");
        let mut pub_use_str = String::from("");
        for name in &self.file_names {
            let snake_case_name = name.to_snake_case();
            mod_str += format!("mod {};\n", &snake_case_name).as_str();
            pub_use_str += format!("pub use {}::*;\n", &snake_case_name).as_str();
        }

        format!("{}\n{}", mod_str, pub_use_str)
    }

    fn path(&self) -> String {
        path_str(vec![self.path, "mod"], true)
    }
}
