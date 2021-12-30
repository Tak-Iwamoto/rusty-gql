use crate::code_generate::FileStrategy;

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

    fn base_path(&self) -> String {
        self.base_path.to_string()
    }

    fn file_name(&self) -> String {
        "mod".to_string()
    }
}
