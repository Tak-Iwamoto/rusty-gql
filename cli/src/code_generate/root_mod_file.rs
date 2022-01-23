use crate::code_generate::FileDefinition;

use super::path_str;

pub struct RootModFile<'a> {
    pub filenames: Vec<String>,
    pub path: &'a str,
}

impl<'a> FileDefinition for RootModFile<'a> {
    fn name(&self) -> String {
        "mod.rs".to_string()
    }

    fn content(&self) -> String {
        let mut mod_str = String::from("");
        let mut pub_use_str = String::from("");
        for name in &self.filenames {
            mod_str += format!("mod {};\n", &name).as_str();
            pub_use_str += format!("pub use {}::*;\n", &name).as_str();
        }

        format!("{}\n{}", mod_str, pub_use_str)
    }

    fn path(&self) -> String {
        path_str(vec![self.path, "mod"], true)
    }
}
